use std::mem::size_of;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::errs::ErrorCode;
use crate::instructions::State;
use crate::instructions::STATE_SEED;
use crate::instructions::events;

pub const PREORDER: &str = "PREORDER";

pub(crate) fn new_preorder(ctx: Context<NewPreorder>, preorder_name: String, amount: u64, price: u64, stm: u64, etm: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    token_interface::transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.preorder_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        }), amount, ctx.accounts.mint.decimals)?;

    ctx.accounts.preorder.stm = stm;
    ctx.accounts.preorder.etm = etm;
    ctx.accounts.preorder.amount = amount;
    ctx.accounts.preorder.price = price;
    ctx.accounts.preorder.mint = ctx.accounts.mint.key();
    ctx.accounts.preorder.collection_mint = ctx.accounts.collection_mint.key();
    ctx.accounts.preorder.extend = [0u64; 32];

    msg!("#new preorder token account: {}, amount: {}", ctx.accounts.payer.key(), amount);

    emit!(events::NewPreorder{
            price,
            amount,
            stm,
            etm,
        });
    Ok(())
}

#[derive(Accounts)]
#[instruction(preorder_name: String)]
pub struct NewPreorder<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(init,
    seeds = [
    PREORDER.as_bytes(),
    preorder_name.as_bytes()
    ],
    bump, payer = payer, space = size_of::<PreOrder>()+8)
    ]
    pub preorder: Box<Account<'info, PreOrder>>,
    // 预售token
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    // 收款token
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = preorder,
        associated_token::token_program = token_program
    )]
    pub preorder_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = collection_mint,
        associated_token::authority = state,
        associated_token::token_program = token_program
    )]
    pub state_collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct PreOrder {
    // 预购总数量
    pub amount: u64,
    // 预购总资金数量
    pub collection_amount: u64,
    // 购买人数
    pub num: u64,
    // 预售开始时间
    pub stm: u64,
    // 预售截止时间
    pub etm: u64,
    pub price: u64,
    // 预购token
    pub mint: Pubkey,
    // 预购收款token
    pub collection_mint: Pubkey,
    // 扩展
    pub extend: [u64; 32],
}