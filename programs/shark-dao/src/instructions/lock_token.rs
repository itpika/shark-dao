use std::mem::size_of;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface;
use anchor_spl::token_interface::{CloseAccount, Mint, TokenAccount};
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, State, events};

const LOCK_INFO_SEED: &str = "lock_info";

pub(crate) fn lock_token(ctx: Context<LockToken>, amount: u64, etm: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    // require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    token_interface::transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.lock_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        }), amount, ctx.accounts.mint.decimals)?;

    ctx.accounts.lock_info.mint = ctx.accounts.mint.key();
    ctx.accounts.lock_info.amount = amount;
    ctx.accounts.lock_info.withdraw = false;
    ctx.accounts.lock_info.owner = ctx.accounts.target_account.key();
    ctx.accounts.lock_info.etm = etm;

    emit!(events::LockToken{
        account: ctx.accounts.target_account.key().to_string(),
        mint: ctx.accounts.mint.key().to_string(),
        amount,
        etm,
    });

    msg!("#lock account: {}, amount: {}, etm: {}, mint: {}", ctx.accounts.target_account.key(), amount, etm, ctx.accounts.mint.key());

    Ok(())
}

pub(crate) fn withdraw_unlock_token(ctx: Context<WithdrawLockToken>) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(!ctx.accounts.lock_info.withdraw, ErrorCode::RepeatedWithdraw);
    require!(!ctx.accounts.lock_info.etm < ctx.accounts.clock.unix_timestamp as u64, ErrorCode::RepeatedWithdraw);

    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.lock_token_account.to_account_info(),
            to: ctx.accounts.payer_token_account.to_account_info(),
            authority: ctx.accounts.lock_info.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        }, &[&[
            LOCK_INFO_SEED.as_bytes(),
            ctx.accounts.mint.key().as_ref(),
            ctx.accounts.payer.key().as_ref(),
            &[ctx.bumps.lock_info]
        ]]), ctx.accounts.lock_token_account.amount, ctx.accounts.mint.decimals)?;

    ctx.accounts.lock_info.withdraw = true;
    ctx.accounts.lock_info.wtm = ctx.accounts.clock.unix_timestamp as u64;

    token_interface::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), CloseAccount {
            account: ctx.accounts.lock_token_account.to_account_info(),
            destination: ctx.accounts.state.to_account_info(),
            authority: ctx.accounts.lock_info.to_account_info(),
        }, &[
            &[
                LOCK_INFO_SEED.as_bytes(),
                ctx.accounts.mint.key().as_ref(),
                ctx.accounts.payer.key().as_ref(),
                &[ctx.bumps.lock_info]
            ],
        ])
    )?;


    msg!("#withdraw account: {} amount: {}", ctx.accounts.payer.key(), ctx.accounts.lock_info.amount);

    emit!(events::WithdrawToken{
        account: ctx.accounts.payer.key().to_string(),
        mint: ctx.accounts.mint.key().to_string(),
        amount: ctx.accounts.lock_info.amount,
    });

    Ok(())
}
#[derive(Accounts)]
pub struct LockToken<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(init_if_needed, seeds = [LOCK_INFO_SEED.as_bytes(), mint.key().as_ref(), target_account.key().as_ref()], bump, payer = payer, space = size_of::<LockInfo>()+8)]
    pub lock_info: Box<Account<'info, LockInfo>>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = lock_info,
        associated_token::token_program = token_program
    )]
    pub lock_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK
    pub target_account: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct WithdrawLockToken<'info> {
    #[account(seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(mut, seeds = [LOCK_INFO_SEED.as_bytes(), mint.key().as_ref(), payer.key().as_ref()], bump)]
    pub lock_info: Box<Account<'info, LockInfo>>,
    #[account(address = lock_info.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = lock_info,
        associated_token::token_program = token_program
    )]
    pub lock_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct LockInfo {
    pub withdraw: bool,
    // 释放时间
    pub etm: u64,
    // 提取时间
    pub wtm: u64,
    pub amount: u64,
    pub mint: Pubkey,
    pub owner: Pubkey,
}