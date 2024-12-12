use std::mem::size_of;
use std::str::FromStr;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer_checked, Token};
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface;
use anchor_spl::token_interface::{Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
// use spl_token::instruction::TokenInstruction::TransferChecked;

use crate::errs::ErrorCode;
use crate::instructions::{events, STATE_SEED, PREORDER, PreOrder, State};

pub const USER_PREORDER: &str = "USER_PREORDER";
pub const USER_PREORDER_SOL: &str = "USER_PREORDER_SOL";

/*
一共分3轮预售，第一轮价格0.005，预售0.6亿枚；第二轮价格0.008，数量0.9亿枚，第三轮价格0.1，数量1亿枚，预售计划一共买出2.5亿枚
*/
// 预购代币
pub(crate) fn preorder_token(ctx: Context<PreorderToken>, preorder_name: String, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);
    let now = ctx.accounts.clock.unix_timestamp as u64;
    require!(ctx.accounts.preorder.stm < now, ErrorCode::TimeOverStm);
    msg!("now {} etm {}", now, ctx.accounts.preorder.etm);
    require!(ctx.accounts.preorder.etm > now, ErrorCode::TimeOver);
    require!(ctx.accounts.preorder_token_account.amount > 0, ErrorCode::InsufficientMintBalance);

    require!(ctx.accounts.user_collection_token_account.amount >= amount, ErrorCode::InsufficientCollectionMintBalance);

    let decimals = 10u64.pow(ctx.accounts.mint.decimals as u32);
    let mut buy_amount = amount.checked_mul(decimals).unwrap().checked_div(ctx.accounts.preorder.price).unwrap();

    require!(buy_amount > 0, ErrorCode::InvalidParameter);

    let mut spend_amont = amount;
    if ctx.accounts.preorder_token_account.amount < buy_amount {
        // 过剩的
        let surplus_amount = buy_amount - ctx.accounts.preorder_token_account.amount;
        buy_amount = ctx.accounts.preorder_token_account.amount;
        spend_amont -= surplus_amount.checked_mul(ctx.accounts.preorder.price).unwrap().checked_div(decimals).unwrap();
    }

    // transfer collection
    token_interface::transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.user_collection_token_account.to_account_info(),
            to: ctx.accounts.state_collection_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.collection_mint.to_account_info(),
        }), spend_amont, ctx.accounts.collection_mint.decimals)?;

    // transfer token
    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.preorder_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.preorder.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        }, &[
            &[
                PREORDER.as_bytes(),
                preorder_name.as_bytes(),
                &[ctx.bumps.preorder]
            ]
        ]), buy_amount, ctx.accounts.mint.decimals)?;


    msg!("#preorder token preorder_name: {} account: {}, token_amount: {}, buy_amount_usdt: {}", preorder_name, ctx.accounts.payer.key(), buy_amount, spend_amont);

    if ctx.accounts.user_preorder.owner.eq(&system_program::id()) {
        ctx.accounts.user_preorder.owner = ctx.accounts.payer.key();
        ctx.accounts.user_preorder.mint = ctx.accounts.mint.key();
        ctx.accounts.user_preorder.collection_mint = ctx.accounts.collection_mint.key();
        ctx.accounts.user_preorder.ctm = ctx.accounts.clock.unix_timestamp as u64;
        ctx.accounts.user_preorder.extend = [0u64; 16];
        ctx.accounts.user_preorder.amount = amount;
        ctx.accounts.user_preorder.buy_amount = buy_amount;
        ctx.accounts.preorder.num += 1;
    } else {
        ctx.accounts.user_preorder.amount += amount;
    }

    ctx.accounts.preorder_token_account.reload().unwrap();
    ctx.accounts.preorder.token_amount = ctx.accounts.preorder_token_account.amount;

    ctx.accounts.preorder.collection_amount += amount;

    emit!(events::Preorder{
            account: ctx.accounts.payer.key().to_string(),
            mint: ctx.accounts.mint.key().to_string(),
            in_amount: amount,
            out_amount: buy_amount,
        });
    Ok(())
}

// sol 购买
pub(crate) fn preorder_token_sol(ctx: Context<PreorderTokenBySol>, preorder_name: String, amount: u64) -> Result<()> {
    require!(ctx.accounts.state.init, ErrorCode::NotInit);

    require!(ctx.accounts.price_update.key().eq(&Pubkey::from_str("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE").unwrap()), ErrorCode::InvalidParameter);

    let now = ctx.accounts.clock.unix_timestamp as u64;
    require!(ctx.accounts.preorder.stm < now, ErrorCode::TimeOverStm);
    // msg!("now {} etm {}", now, ctx.accounts.preorder.etm);
    require!(ctx.accounts.preorder.etm > now, ErrorCode::TimeOver);

    require!(ctx.accounts.payer.lamports() >= amount, ErrorCode::InsufficientCollectionMintBalance);
    require!(ctx.accounts.preorder_token_account.amount > 0, ErrorCode::InsufficientMintBalance);

    let price_update = &mut ctx.accounts.price_update;
    // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 30;
    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
    let feed_id: [u8; 32] = get_feed_id_from_hex("0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d")?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
    // The price is (7160106530699 ± 5129162301) * 10^-8
    msg!("The price is ({} ± {}) * {}", price.price, price.conf, price.exponent);

    let mut sol_price: u64;
    if price.exponent < 0 {
        sol_price = price.price.checked_div(10_i32.pow((price.exponent *-1) as u32) as i64).unwrap() as u64;
    } else {
        sol_price = price.price.checked_mul(10_i32.pow((price.exponent) as u32) as i64).unwrap() as u64;
    }
    msg!("sol_price: {}", sol_price);


    // 避免溢出，sol和 token精度同时除1e6
    let sol_decimals = 1000000000_u64.checked_div(1000000).unwrap();
    // let sol_price = ctx.accounts.state.sol_price.checked_div(1000000).unwrap();

    // compute USD price
    let u_amount = amount.checked_mul(sol_price).unwrap().checked_div(sol_decimals).unwrap();

    let decimals = 10u64.pow(ctx.accounts.mint.decimals as u32);
    let mut buy_amount = u_amount.checked_mul(decimals).unwrap().checked_div(ctx.accounts.preorder.price).unwrap();

    require!(buy_amount > 0, ErrorCode::InvalidParameter);

    let mut spend_amont = amount;
    if ctx.accounts.preorder_token_account.amount < buy_amount {
        // 过剩的
        let surplus_amount = buy_amount - ctx.accounts.preorder_token_account.amount;
        buy_amount = ctx.accounts.preorder_token_account.amount;

        let surplus_u_amount = surplus_amount.checked_mul(ctx.accounts.preorder.price).unwrap().checked_div(decimals).unwrap();

        spend_amont -= surplus_u_amount.checked_mul(sol_decimals).unwrap().checked_div(sol_price).unwrap()
    }

    // transfer sol
    transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.state.to_account_info(),
    }), spend_amont)?;

    // transfer token
    token_interface::transfer_checked(
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),TransferChecked {
            from: ctx.accounts.preorder_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.preorder.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        }, &[
            &[
                PREORDER.as_bytes(),
                preorder_name.as_bytes(),
                &[ctx.bumps.preorder]
            ]
        ]), buy_amount, ctx.accounts.mint.decimals)?;



    msg!("#preorder token preorder_name: {} account: {}, token_amount: {}, buy_amount_sol: {}", preorder_name, ctx.accounts.payer.key(), buy_amount,spend_amont);

    if ctx.accounts.user_preorder.owner.eq(&system_program::id()) {
        ctx.accounts.user_preorder.owner = ctx.accounts.payer.key();
        ctx.accounts.user_preorder.mint = ctx.accounts.mint.key();
        ctx.accounts.user_preorder.ctm = ctx.accounts.clock.unix_timestamp as u64;
        ctx.accounts.user_preorder.extend = [0u64; 16];
        ctx.accounts.user_preorder.amount = amount;
        ctx.accounts.user_preorder.buy_amount = buy_amount;
        ctx.accounts.preorder.num += 1;
    } else {
        ctx.accounts.user_preorder.amount += amount;
    }

    ctx.accounts.preorder_token_account.reload().unwrap();
    ctx.accounts.preorder.token_amount = ctx.accounts.preorder_token_account.amount;

    ctx.accounts.preorder.sol_amount += amount;

    emit!(events::Preorder{
            account: ctx.accounts.payer.key().to_string(),
            mint: ctx.accounts.system_program.key().to_string(),
            in_amount: amount,
            out_amount: buy_amount,
        });
    Ok(())
}

#[derive(Accounts)]
#[instruction(preorder_name: String)]
pub struct PreorderToken<'info> {
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(init_if_needed, seeds = [USER_PREORDER.as_bytes(), preorder.key().as_ref(), payer.key().as_ref()], bump, payer = payer, space = size_of::<UserPreOrder>()+8)]
    pub user_preorder: Box<Account<'info, UserPreOrder>>,
    #[account(mut,
    seeds = [
    PREORDER.as_bytes(),
    preorder_name.as_bytes()
    ],
    bump)
    ]
    pub preorder: Box<Account<'info, PreOrder>>,
    // 预售token
    #[account(address = preorder.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    // 收款token
    #[account(address = preorder.collection_mint)]
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = preorder,
        associated_token::token_program = token_program
    )]
    pub preorder_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = collection_mint,
        associated_token::authority = state,
        associated_token::token_program = token_program
    )]
    pub state_collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = collection_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub user_collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(preorder_name: String)]
pub struct PreorderTokenBySol<'info> {
    pub price_update: Account<'info, PriceUpdateV2>,
    #[account(mut, seeds = [STATE_SEED.as_bytes()], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(init_if_needed, seeds = [USER_PREORDER_SOL.as_bytes(), preorder.key().as_ref(), payer.key().as_ref()], bump, payer = payer, space = size_of::<UserSolPreOrder>()+8)]
    pub user_preorder: Box<Account<'info, UserSolPreOrder>>,
    #[account(mut,
    seeds = [
    PREORDER.as_bytes(),
    preorder_name.as_bytes()
    ],
    bump)
    ]
    pub preorder: Box<Account<'info, PreOrder>>,
    // 预售token
    #[account(address = preorder.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = preorder,
        associated_token::token_program = token_program
    )]
    pub preorder_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


#[account]
pub struct UserPreOrder {
    // 花费token数量
    pub amount: u64,
    // 购买数量
    pub buy_amount: u64,
    // 预售时间
    pub ctm: u64,
    pub mint: Pubkey,
    pub collection_mint: Pubkey,
    pub owner: Pubkey,
    // 扩展
    pub extend: [u64; 16],
}

#[account]
pub struct UserSolPreOrder {
    // 花费Sol数量
    pub amount: u64,
    // 购买数量
    pub buy_amount: u64,
    // 预售时间
    pub ctm: u64,
    pub mint: Pubkey,
    pub owner: Pubkey,
    // 扩展
    pub extend: [u64; 16],
}