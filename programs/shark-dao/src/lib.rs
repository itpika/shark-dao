use anchor_lang::prelude::*;

declare_id!("BQ4h4xZ3v6VmCdsvkPMuAyPPD7TUo8DVXEdHH8ZjZGjQ");

#[program]
pub mod shark_dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
