use anchor_lang::prelude::*;

#[event]
pub struct Preorder{
    pub account: String,
    pub amount: u64,
}

#[event]
pub struct WithdrawToken{
    pub account: String,
    pub mint: String,
    pub amount: u64,
}

#[event]
pub struct WithdrawSol{
    pub account: String,
    pub amount: u64,
}
