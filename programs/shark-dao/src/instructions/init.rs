use std::mem::size_of;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::errs::ErrorCode;

pub const STATE_SEED: &str = "state";

pub(crate) fn init(ctx: Context<Init>) -> Result<()> {
    require!(!ctx.accounts.state.init, ErrorCode::RepeatedInit);
    ctx.accounts.state.admin = ctx.accounts.payer.key();
    msg!("init admin: {}", ctx.accounts.payer.key());
    ctx.accounts.state.init = true;
    ctx.accounts.state.rate = 0;
    ctx.accounts.state.num = 0;
    ctx.accounts.state.admin = ctx.accounts.payer.key();
    ctx.accounts.state.extend = [0_u64; 32];
    Ok(())
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, seeds = [STATE_SEED.as_bytes()], bump, payer = payer, space = size_of::<State>()+8)]
    pub state: Box<Account<'info, State>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    pub init: bool,
    // 代币兑换比例
    pub rate: u16,
    // 预购人数
    pub num: u64,
    // 管理员地址
    pub admin: Pubkey,
    // token
    pub mint: Pubkey,
    pub extend: [u64; 32]
}

impl State {
    pub fn default() -> Self {
        State {
            init: false,
            rate: 0,
            num: 0,
            admin: Pubkey::default(),
            mint: Pubkey::default(),
            extend: [0;32]
        }
    }
}