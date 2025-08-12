use anchor_lang::prelude::*;
use crate::state::SpendingAccount;
use crate::error::AmountError;

pub fn authorize_spend(ctx: Context<AuthorizeSpend>, amount: u64) -> Result<()> {
    ctx.accounts.authorize(amount)
}

#[derive(Accounts)]
pub struct AuthorizeSpend<'info> {

    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"spending", authority.key().as_ref()],
        bump = spending_account.bump
    )]
    pub spending_account: Account<'info, SpendingAccount>,
}

impl<'info> AuthorizeSpend<'info> {
    pub fn authorize(&mut self, amount: u64) -> Result<()> {
        
        let available_to_spend = self.spending_account.spending_limit
            .checked_sub(self.spending_account.amount_spent)
            .unwrap();

        
        require!(
            amount <= available_to_spend,
            AmountError::SpendingLimitExceeded
        );

        self.spending_account.amount_spent = self.spending_account.amount_spent
            .checked_add(amount)
            .unwrap();

        Ok(())
    }
}
