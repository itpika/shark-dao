use anchor_lang::prelude::*;

#[event]
pub struct NewPreorder{
    pub price: u64,
    pub amount: u64,
    pub stm: u64,
    pub etm: u64,
}

#[event]
pub struct Preorder{
    pub account: String,
    pub in_amount: u64,
    pub out_amount: u64,
}

#[event]
pub struct WithdrawToken{
    pub account: String,
    pub mint: String,
    pub amount: u64,
}

#[event]
pub struct LockToken{
    pub account: String,
    pub mint: String,
    pub amount: u64,
    pub etm: u64,
}

#[event]
pub struct WithdrawFund{
    pub account: String,
    pub mint: String,
    pub amount: u64,
}

#[event]
pub struct WithdrawSol{
    pub account: String,
    pub amount: u64,
}
