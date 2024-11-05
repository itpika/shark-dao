use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, State};


pub(crate) fn set_mint(ctx: Context<SetMint>) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);
    msg!("set mint: {}", ctx.accounts.mint.key());
    ctx.accounts.state.mint = ctx.accounts.mint.key();
    Ok(())
}

#[derive(Accounts)]
pub struct SetMint<'info> {
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
