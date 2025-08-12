use anchor_lang::prelude::*;
use crate::{error::AmountError, state::Vault};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw_sol(&mut self, amount: u64) -> Result<()> {
        // Check if we have sufficient balance in our tracker
        require!(
            self.vault.balance >= amount,
            AmountError::InsufficientFunds
        );

        // Get rent exemption minimum for the vault account
        let rent = Rent::get()?;
        let vault_data_len = 8 + std::mem::size_of::<Vault>(); // 8 bytes for discriminator + Vault size
        let min_rent_exempt_balance = rent.minimum_balance(vault_data_len);
        
        let current_lamports = self.vault.to_account_info().lamports();
        
        // Ensure we don't withdraw below rent exemption
        require!(
            current_lamports.checked_sub(amount).unwrap_or(0) >= min_rent_exempt_balance,
            AmountError::InsufficientFunds
        );

        // Perform the transfer using proper CPI to system program
        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: self.vault.to_account_info(),
            to: self.authority.to_account_info(),
        };

        let binding = self.authority.key();
        let vault_seeds = &[
            b"vault",
            binding.as_ref(),
            &[self.vault.bump], // You'll need to store bump in your Vault struct
        ];
        let signer_seeds = &[&vault_seeds[..]];

        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                transfer_instruction,
                signer_seeds,
            ),
            amount,
        )?;

        // Update our vault's internal balance tracker
        self.vault.balance = self.vault
            .balance
            .checked_sub(amount)
            .ok_or(AmountError::InsufficientFunds)?;

        Ok(())
    }
}

// You'll also need to update your Vault struct to store the bump:
// #[account]
// pub struct Vault {
//     pub authority: Pubkey,
//     pub balance: u64,
//     pub bump: u8,  // Add this field
// }