use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("5ceSNwkPDqM2rxtQgCVdEaTMXvwdL3jP4dqoPA5oe6be");

#[program]
pub mod week2a_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        Ok(())
    }
}

impl<'info> Payment<'info> {
    pub fn deposit (&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts:Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw (&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts:Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[b"vault", self.state.to_account_info().key.as_ref()];

        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Vaultstate::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, Vaultstate>,

    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]

pub struct Payment<'info> {
    #[account(mut)] 
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump,
    )]
    pub state: Account<'info, Vaultstate>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct Close<'info> {
    #[account(mut)] 
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump,
        close = user,
    )]
    pub state: Account<'info, Vaultstate>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let balance = self.vault.get_lamports();
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts:Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[b"vault", self.state.to_account_info().key.as_ref()];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer(cpi_ctx, balance)?;
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]

pub struct Vaultstate {
    pub vault_bump: u8,
    pub state_bump: u8,
}