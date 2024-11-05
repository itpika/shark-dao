use anchor_lang::prelude::*;

#[event]
pub struct Preorder{
    pub account: String,
    pub amount: u64,
}
