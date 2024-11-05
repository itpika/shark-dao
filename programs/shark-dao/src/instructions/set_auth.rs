use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, State};


pub(crate) fn set_auth(ctx: Context<SetAuth>, admin: Pubkey) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);
    msg!("set auth: {} -> {}", ctx.accounts.payer.key(), admin);
    ctx.accounts.state.admin = admin;
    Ok(())
}

#[derive(Accounts)]
pub struct SetAuth<'info> {
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
