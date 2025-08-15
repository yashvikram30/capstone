use anchor_lang::prelude::*;
use crate::state::SpendingAccount;

#[derive(Accounts)]
pub struct ResetSpendTracker<'info> {

    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"spending", authority.key().as_ref()],
        bump = spending_account.bump
    )]
    pub spending_account: Account<'info, SpendingAccount>,
}

impl <'info> ResetSpendTracker<'info> {
    
    pub fn reset_spend_tracker(&mut self) -> Result<()> {
        self.spending_account.amount_spent = 0;
        Ok(())
    }
}