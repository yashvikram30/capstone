use anchor_lang::prelude::*;

use crate::{SpendingAccount, Vault};

#[derive(Accounts)]
pub struct UpdateSpendingLimit<'info> {

    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"spending", authority.key().as_ref()],
        bump
    )]
    pub spending_account: Account<'info, SpendingAccount>,

}

impl <'info> UpdateSpendingLimit <'info> {

    pub fn update_spending_limit(&mut self) -> Result<()>{

        let new_limit = self.vault.balance / 2;
        self.spending_account.spending_limit = new_limit;

        Ok(())
    }
}