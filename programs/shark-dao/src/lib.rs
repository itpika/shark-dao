mod errs;
mod instructions;
use instructions::*;
use anchor_lang::prelude::*;

declare_id!("3vHSdBG1jZxq8r39y1NH4Mts6LyccMPs6WX6u2ic2BE1");


#[program]
pub mod shark_dao {
    use super::*;
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }
    pub fn new_preorder(ctx: Context<NewPreorder>, preorder_name: String, amount: u64, price: u64, stm: u64, etm: u64) -> Result<()> {
        instructions::new_preorder(ctx, preorder_name, amount, price, stm, etm)
    }
    pub fn preorder_token(ctx: Context<PreorderToken>, preorder_name: String, amount: u64) -> Result<()> {
        instructions::preorder_token(ctx, preorder_name, amount)
    }
    pub fn preorder_token_sol(ctx: Context<PreorderTokenBySol>, preorder_name: String, amount: u64) -> Result<()> {
        instructions::preorder_token_sol(ctx, preorder_name, amount)
    }
    pub fn set_auth(ctx: Context<SetAuth>, admin: Pubkey) -> Result<()> {
        instructions::set_auth(ctx, admin)
    }
    pub fn set_sol_price(ctx: Context<SetAuth>, price: u64) -> Result<()> {
        instructions::set_sol_price(ctx, price)
    }

    pub fn withdraw_collection_token(ctx: Context<WithdrawCollectionToken>, amount: u64) -> Result<()> {
        instructions::withdraw_collection_token(ctx, amount)
    }
    // pub fn withdraw_fund(ctx: Context<WithdrawFund>, amount: u64) -> Result<()> {
    //     instructions::withdraw_fund(ctx, amount)
    // }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        instructions::withdraw_sol(ctx, amount)
    }
    pub fn lock_token(ctx: Context<LockToken>, amount: u64, etm: u64) -> Result<()> {
        instructions::lock_token(ctx, amount, etm)
    }
    pub fn withdraw_unlock_token(ctx: Context<WithdrawLockToken>) -> Result<()> {
        instructions::withdraw_unlock_token(ctx)
    }

}
