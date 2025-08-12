use anchor_lang::prelude::*;

use crate::Treasury;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program : Program<'info, System>,

}

impl <'info> InitializeTreasury <'info> {
    
    pub fn initialize_treasury(&mut self, bumps: &InitializeTreasuryBumps) -> Result<()>{

        self.treasury.set_inner(Treasury { bump: bumps.treasury });
        Ok(())
    }
}