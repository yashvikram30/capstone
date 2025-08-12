use anchor_lang::prelude::*;

use crate::SpendingAccount;

#[derive(Accounts)]
pub struct InitializeSpendingAccount<'info>{

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + SpendingAccount::INIT_SPACE,
        seeds = [b"spending", authority.key().as_ref()],
        bump
    )]
    pub spending_account: Account<'info, SpendingAccount>,

    pub system_program: Program<'info, System>,
}

impl <'info> InitializeSpendingAccount <'info> {
    
    pub fn init_spending_account(&mut self, bumps: &InitializeSpendingAccountBumps) -> Result<()>{

        self.spending_account.set_inner(SpendingAccount { 
            authority: self.authority.key(), 
            spending_limit: 0, 
            amount_spent: 0,
            bump: bumps.spending_account 
        });

        Ok(())
    }
}