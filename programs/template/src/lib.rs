use anchor_lang::prelude::*;
use lib::print;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod template {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello {}!", print());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
