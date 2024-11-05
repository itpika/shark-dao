mod errs;
mod instructions;
use instructions::*;
use anchor_lang::prelude::*;

declare_id!("BQ4h4xZ3v6VmCdsvkPMuAyPPD7TUo8DVXEdHH8ZjZGjQ");


#[program]
pub mod shark_dao {
    use super::*;
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn preorder_token(ctx: Context<PreorderToken>, amount: u64) -> Result<()> {
        instructions::preorder_token(ctx, amount)
    }
    pub fn set_auth(ctx: Context<SetAuth>, admin: Pubkey) -> Result<()> {
        instructions::set_auth(ctx, admin)
    }
    pub fn set_mint(ctx: Context<SetMint>) -> Result<()> {
        instructions::set_mint(ctx)
    }
    pub fn withdraw_token(ctx: Context<WithdrawToken>) -> Result<()> {
        instructions::withdraw_token(ctx)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        instructions::withdraw_sol(ctx, amount)
    }


    // 管理员提取
    // pub fn withdraw(ctx: Context<Withdraw>, id: String) -> Result<()> {
    //
    //     require!(ctx.accounts.payer.key().eq(&ctx.accounts.pro.admin), errs::ErrorCode::NotAuthorized);
    //     let amount = ctx.accounts.from_token_account.amount;
    //     require!(amount > 0, errs::ErrorCode::InsufficientBalance);
    //
    //
    //     msg!("#withdraw {} {} amount {}", ctx.accounts.payer.key(), id, amount);
    //
    //     token_interface::transfer_checked(
    //         CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
    //             from: ctx.accounts.from_token_account.to_account_info(),
    //             to: ctx.accounts.token_account.to_account_info(),
    //             authority: ctx.accounts.pro.to_account_info(),
    //             mint: ctx.accounts.mint.to_account_info(),
    //         }, &[&[
    //             PROJECT_SEED.as_bytes(),
    //             id.as_bytes(),
    //             &[ctx.bumps.pro]
    //         ]]), ctx.accounts.from_token_account.amount, ctx.accounts.mint.decimals)?;
    //
    //     Ok(())
    // }
}


// #[derive(Accounts)]
// #[instruction(id: String)]
// pub struct NewProject<'info> {
//     #[account(init, seeds = [PROJECT_SEED.as_bytes(), id.as_bytes()], bump, payer = payer, space = size_of::<Project>()+8)]
//     pub pro: Box<Account<'info, Project>>,
//     #[account(seeds = [ADMIN_SEED.as_bytes()], bump)]
//     pub state: Box<Account<'info, State>>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     pub clock: Sysvar<'info, Clock>,
//     pub mint: Box<InterfaceAccount<'info, Mint>>,
//     #[account(
//         init,
//         payer = payer,
//         associated_token::mint = mint,
//         associated_token::authority = pro,
//         associated_token::token_program = token_program
//     )]
//     pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     #[account(address = spl_token::id())]
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>
// }
//
// #[derive(Accounts)]
// #[instruction(id: String)]
// pub struct Deposit<'info> {
//     #[account(mut, seeds = [PROJECT_SEED.as_bytes(), id.as_bytes()], bump)]
//     pub pro: Box<Account<'info, Project>>,
//     #[account(seeds = [ADMIN_SEED.as_bytes()], bump)]
//     pub state: Box<Account<'info, State>>,
//     #[account(init_if_needed, seeds = [DEPOSIT_SEED.as_bytes(), id.as_bytes(), payer.key().as_ref()], bump, payer = payer, space = size_of::<Personal>()+8)]
//     pub personal: Box<Account<'info, Personal>>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     #[account(address = pro.mint)]
//     pub mint: Box<InterfaceAccount<'info, Mint>>,
//     #[account(
//         mut,
//         associated_token::mint = mint,
//         associated_token::authority = payer,
//         associated_token::token_program = token_program
//     )]
//     pub from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     #[account(
//         mut,
//         associated_token::mint = mint,
//         associated_token::authority = pro,
//         associated_token::token_program = token_program
//     )]
//     pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     #[account(address = spl_token::id())]
//     pub token_program: Program<'info, Token>,
// }
//
// #[derive(Accounts)]
// #[instruction(id: String)]
// pub struct Withdraw<'info> {
//     #[account(mut, seeds = [PROJECT_SEED.as_bytes(), id.as_bytes()], bump)]
//     pub pro: Box<Account<'info, Project>>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     #[account(address = pro.mint)]
//     pub mint: Box<InterfaceAccount<'info, Mint>>,
//     #[account(
//         mut,
//         associated_token::mint = mint,
//         associated_token::authority = pro,
//         associated_token::token_program = token_program
//     )]
//     pub from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     #[account(
//         init_if_needed,
//         associated_token::mint = mint,
//         payer = payer,
//         associated_token::authority = payer,
//         associated_token::token_program = token_program
//     )]
//     pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     #[account(address = spl_token::id())]
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>
// }

