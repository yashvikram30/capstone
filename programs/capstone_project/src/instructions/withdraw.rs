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
        let vault_data_len = 8 + std::mem::size_of::<Vault>();
        let min_rent_exempt_balance = rent.minimum_balance(vault_data_len);
        
        let current_lamports = self.vault.to_account_info().lamports();
        
        // Ensure we don't withdraw below rent exemption
        require!(
            current_lamports.checked_sub(amount).unwrap_or(0) >= min_rent_exempt_balance,
            AmountError::InsufficientFunds
        );

        // Manually transfer lamports from vault to user
        // We need to use the vault's authority to sign this transfer
        let vault_info = self.vault.to_account_info();
        let authority_info = self.authority.to_account_info();
        
        // Debit from vault
        **vault_info.try_borrow_mut_lamports()? = vault_info
            .lamports()
            .checked_sub(amount)
            .ok_or(AmountError::InsufficientFunds)?;
        
        // Credit to authority
        **authority_info.try_borrow_mut_lamports()? = authority_info
            .lamports()
            .checked_add(amount)
            .unwrap();

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