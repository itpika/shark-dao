use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, USER_PREORDER, State, UserPreOrder, events, PreOrder};


pub(crate) fn withdraw_token(ctx: Context<WithdrawToken>) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.user_preorder.owner.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);


    emit!(events::WithdrawToken{
        account: ctx.accounts.payer.key().to_string(),
        mint: ctx.accounts.mint.key().to_string(),
        amount: 0,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(mut, seeds = [USER_PREORDER.as_bytes(), payer.key().as_ref()], bump)]
    pub user_preorder: Box<Account<'info, UserPreOrder>>,
    #[account(address = user_preorder.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = state,
        associated_token::token_program = token_program
    )]
    pub state_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}
