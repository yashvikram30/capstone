use anchor_lang::prelude::*;

use crate::{Treasury, YieldAccount};

#[derive(Accounts)]
pub struct InitializeYieldAccount<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + YieldAccount::INIT_SPACE,
        seeds = [b"yield", authority.key().as_ref()],
        bump
    )]
    pub yield_account: Account<'info, YieldAccount>,

    pub system_program : Program<'info, System>,

}

impl <'info> InitializeYieldAccount <'info> {
    
    pub fn initialize_yield_account(&mut self, bumps: &InitializeYieldAccountBumps) -> Result<()>{

        self.yield_account.set_inner(YieldAccount { 
            authority: self.authority.key(), 
            staked_amount: 0, 
            bump: bumps.yield_account 
        });
        Ok(())
    }
}