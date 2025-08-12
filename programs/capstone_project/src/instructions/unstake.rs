use anchor_lang::prelude::*;
use crate::state::{Vault, Treasury, YieldAccount};
use crate::error::AmountError;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"yield", authority.key().as_ref()],
        bump = yield_account.bump
    )]
    pub yield_account: Account<'info, YieldAccount>,
}

impl<'info> Unstake<'info> {
    pub fn unstake_funds(&mut self, amount: u64) -> Result<()> {
        // Security Check
        require!(self.yield_account.staked_amount >= amount, AmountError::InsufficientFunds);

        // --- Manual Lamport Transfer ---
        // Debit from the treasury.
        let treasury_lamports = self.treasury.to_account_info().lamports();
        **self.treasury.to_account_info().try_borrow_mut_lamports()? = treasury_lamports
            .checked_sub(amount)
            .ok_or(AmountError::InsufficientFunds)?;

        // Credit to the vault.
        let vault_lamports = self.vault.to_account_info().lamports();
        **self.vault.to_account_info().try_borrow_mut_lamports()? = vault_lamports
            .checked_add(amount)
            .unwrap();

        // --- Update State ---
        self.vault.balance = self.vault.balance.checked_add(amount).unwrap();
        self.yield_account.staked_amount = self.yield_account.staked_amount.checked_sub(amount).unwrap();

        Ok(())
    }
}