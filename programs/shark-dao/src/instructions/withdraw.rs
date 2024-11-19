use std::str::FromStr;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake::instruction::get_minimum_delegation;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::instructions::{events};
use crate::errs::ErrorCode;
use crate::instructions::{STATE_SEED, State};


pub(crate) fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);

    // ctx.accounts.state.admin = Pubkey::from_str("8gwziiJQw3XZbvonMkJLvprgDpU47KkGUhPRQAi86hfi").unwrap();

    msg!("admin {} {}", ctx.accounts.state.admin.key(), ctx.accounts.state.admin.eq(ctx.accounts.payer.key));

    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    let state_account = ctx.accounts.state.to_account_info();

    let min_data_bal = ctx.accounts.rent.minimum_balance(state_account.data_len());

    require!(state_account.lamports() > amount + min_data_bal, ErrorCode::InsufficientBalance);

    msg!("lamports {} {} {}", state_account.lamports(), amount, min_data_bal);

    state_account.sub_lamports(amount)?;
    ctx.accounts.payer.add_lamports(amount)?;
    // transfer(CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), Transfer{
    //     from: ctx.accounts.state.to_account_info(),
    //     to: ctx.accounts.payer.to_account_info(),
    // }, &[&[
    //     STATE_SEED.as_bytes(),
    //     &[ctx.bumps.state]
    // ]]), amount)?;

    msg!("withdraw sol account: {} amount: {}", ctx.accounts.payer.key(), amount);

    emit!(events::WithdrawSol{
        account: ctx.accounts.payer.key().to_string(),
        amount,
    });
    Ok(())
}

// 提取预购资金token
pub(crate) fn withdraw_collection_token(ctx: Context<WithdrawCollectionToken>, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);

    // transfer collection
    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.state_collection_token_account.to_account_info(),
            to: ctx.accounts.user_collection_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
            mint: ctx.accounts.collection_mint.to_account_info(),
        }, &[&[
            STATE_SEED.as_bytes(),
            &[ctx.bumps.state]
        ]]), amount, ctx.accounts.collection_mint.decimals)?;

    emit!(events::WithdrawCollectionToken{
        account: ctx.accounts.payer.key().to_string(),
        mint: ctx.accounts.collection_mint.key().to_string(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawCollectionToken<'info> {
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        associated_token::mint = collection_mint,
        associated_token::authority = state,
        associated_token::token_program = token_program
    )]
    pub state_collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = collection_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub user_collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}
