use std::mem::size_of;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::errs::ErrorCode;

pub const STATE_SEED: &str = "state";

pub(crate) fn init(ctx: Context<Init>) -> Result<()> {
    require!(!ctx.accounts.state.init, ErrorCode::RepeatedInit);
    msg!("init admin: {}", ctx.accounts.payer.key());
    ctx.accounts.state.init = true;
    ctx.accounts.state.admin = ctx.accounts.payer.key();
    ctx.accounts.state.sol_price = 500*1000000; // default 500 USD
    // ctx.accounts.state.extend = [0_u64; 32];
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
    // 管理员地址
    pub admin: Pubkey,
    pub sol_price: u64,
    pub extend: [u64; 31]
}

// impl State {
//     pub fn default() -> Self {
//         State {
//             init: false,
//             admin: Pubkey::default(),
//             sol_price: 0,
//             extend: [0;32],
//         }
//     }
// }