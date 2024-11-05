use std::mem::size_of;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::system_program::{transfer, Transfer};
use crate::errs::ErrorCode;
use crate::instructions::State;
use crate::instructions::STATE_SEED;
use crate::instructions::events;

pub const USER_PREORDER: &str = "USER_PREORDER";

// 预购代币
pub(crate) fn preorder_token(ctx: Context<PreorderToken>, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.payer.lamports() > amount, ErrorCode::InsufficientBalance);

    transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.state.to_account_info(),
    }), amount)?;
    msg!("preorder token account: {}, amount: {}", ctx.accounts.payer.key(), amount);

    if ctx.accounts.user_preorder.owner.eq(&system_program::id()) {
        ctx.accounts.user_preorder.owner = ctx.accounts.payer.key();
        ctx.accounts.user_preorder.ctm = ctx.accounts.clock.unix_timestamp as u64;
        ctx.accounts.user_preorder.extend = [0u64; 16];
    }
    ctx.accounts.user_preorder.amount += amount;
    ctx.accounts.state.num += 1;


    emit!(events::Preorder{
            account: ctx.accounts.payer.key().to_string(),
            amount,
        });
    Ok(())
}

#[derive(Accounts)]
pub struct PreorderToken<'info> {
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(init_if_needed, seeds = [USER_PREORDER.as_bytes(), payer.key().as_ref()], bump, payer = payer, space = size_of::<UserPreOrder>()+8)]
    pub user_preorder: Box<Account<'info, UserPreOrder>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct UserPreOrder {
    // 预购数量
    pub amount: u64,
    // 预售时间
    pub ctm: u64,
    pub owner: Pubkey,
    // 扩展
    pub extend: [u64; 16],
}