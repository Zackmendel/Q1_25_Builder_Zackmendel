use anchor_lang::prelude::*;

declare_id!("KVLv7MiyNnMJMTdRFCsTN9FEqBc7i198NY8kDcXvBsk");

mod instructions;
mod state;

use crate::instructions::*;

// #[program]
pub mod week2a_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, bump: &MakeBumps, deposit:u64 ) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, bump)?;
        ctx.accounts.deposit(deposit);

        Ok(())
    }

    pub fn take(ctx: Context<Take>, amount: u64) -> Result<()> {
        ctx.accounts.send(amount)?;
        ctx.accounts.withdraw(amount)?;
        ctx.accounts.close()?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        ctx.accounts.refund_and_close_vault(amount)?;

        Ok(())
    }

    
}

// #[derive(Accounts)]
// pub struct Initialize {}
