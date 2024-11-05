use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::instructions::events;
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, USER_PREORDER, State, UserPreOrder};


pub(crate) fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    transfer(CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), Transfer{
        from: ctx.accounts.state.to_account_info(),
        to: ctx.accounts.payer.to_account_info(),
    }, &[&[
        STATE_SEED.as_bytes(),
        &[ctx.bumps.state]
    ]]), amount)?;

    msg!("withdraw sol account: {} amount: {}", ctx.accounts.payer.key(), amount);

    emit!(events::WithdrawSol{
        account: ctx.accounts.payer.key().to_string(),
        amount,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

}
