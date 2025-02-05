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
use crate::instructions::new_preorder::PreOrder;
use crate::instructions::new_preorder::PREORDER;
use crate::instructions::STATE_SEED;
use crate::instructions::events;


pub(crate) fn get_back(ctx: Context<GetBack>, preorder_name: String) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.preorder_token_account.to_account_info(),
            to: ctx.accounts.payer_token_account.to_account_info(),
            authority: ctx.accounts.preorder.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        }, &[
            &[
                PREORDER.as_bytes(),
                preorder_name.as_bytes(),
                &[ctx.bumps.preorder]
            ]
        ]), ctx.accounts.preorder_token_account.amount, ctx.accounts.mint.decimals)?;

    msg!("#get back : {}, amount: {}", preorder_name.clone(), ctx.accounts.preorder_token_account.amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(preorder_name: String)]
pub struct GetBack<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(
    seeds = [
    PREORDER.as_bytes(),
    preorder_name.as_bytes()
    ],
    bump)
    ]
    pub preorder: Box<Account<'info, PreOrder>>,
    // 预售token
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = preorder,
        associated_token::token_program = token_program
    )]
    pub preorder_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
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

