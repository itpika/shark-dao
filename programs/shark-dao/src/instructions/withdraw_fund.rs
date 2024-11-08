use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, State, events};


pub(crate) fn withdraw_fund(ctx: Context<WithdrawFund>, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    // transfer token
    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.state_token_account.to_account_info(),
            to: ctx.accounts.payer_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
            mint: ctx.accounts.collection_mint.to_account_info(),
        }, &[
            &[
                STATE_SEED.as_bytes(),
                &[ctx.bumps.state]
            ]
        ]), amount, ctx.accounts.collection_mint.decimals)?;

    msg!("#withdraw fund account: {} amount: {}",ctx.accounts.payer.key() , amount);

    emit!(events::WithdrawToken{
        account: ctx.accounts.payer.key().to_string(),
        mint: ctx.accounts.collection_mint.key().to_string(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFund<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        associated_token::mint = collection_mint,
        associated_token::authority = state,
        associated_token::token_program = token_program
    )]
    pub state_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = collection_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}
