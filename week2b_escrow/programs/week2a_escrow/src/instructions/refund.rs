use anchor_lang::prelude::*;
use anchor_spl::{self, 
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account},
    };

use crate::state::escrow::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    // pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        from: self.vault.to_account_info(),
        to: self.maker_ata_a.to_account_info(),
        authority: self.escrow.to_account_info(),
        mint: self.mint_a.to_account_info(),
    };

    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"escrow",
        self.maker.key.as_ref(),
        &self.escrow.seed.to_le_bytes()[..],
        &[self.escrow.bump],
    ]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

    transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = CloseAccount {
        account: self.vault.to_account_info(),
        destination: self.maker_ata_a.to_account_info(),
        authority: self.escrow.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

    close_account(cpi_ctx)?;

    Ok(())
    }
}