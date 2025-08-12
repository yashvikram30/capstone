use anchor_lang::prelude::*;
use crate::state::{Vault, Treasury, YieldAccount};
use crate::error::AmountError;

#[derive(Accounts)]
pub struct Stake<'info> {
    // The user authorizing the stake.
    #[account(mut)]
    pub authority: Signer<'info>,

    // The user's vault to withdraw funds from.
    #[account(
        mut,
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    // The central treasury to deposit funds into.
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    // The user's yield account to track their stake.
    #[account(
        mut,
        has_one = authority,
        seeds = [b"yield", authority.key().as_ref()],
        bump = yield_account.bump
    )]
    pub yield_account: Account<'info, YieldAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake_funds(&mut self, amount: u64) -> Result<()> {
        // --- Security Check ---
        // Ensure the user's vault has enough funds to stake.
        require!(self.vault.balance >= amount, AmountError::InsufficientFunds);

        // --- Lamport Transfer ---
        // We manually move the lamports from the user's vault PDA to the treasury PDA.
        // This is the same pattern we used in the `withdraw` instruction.

        // 1. Debit from the vault.
        let vault_lamports = self.vault.to_account_info().lamports();
        **self.vault.to_account_info().try_borrow_mut_lamports()? = vault_lamports
            .checked_sub(amount)
            .ok_or(AmountError::InsufficientFunds)?;

        // 2. Credit to the treasury.
        let treasury_lamports = self.treasury.to_account_info().lamports();
        **self.treasury.to_account_info().try_borrow_mut_lamports()? = treasury_lamports
            .checked_add(amount)
            .unwrap();

        // --- Update State ---
        // Now we update our own account records to reflect the transfer.
        self.vault.balance = self.vault.balance.checked_sub(amount).unwrap();
        self.yield_account.staked_amount = self.yield_account.staked_amount.checked_add(amount).unwrap();

        Ok(())
    }
}
