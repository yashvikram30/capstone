use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct Initialize<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
    pub fn init_vault(&mut self, bump: &InitializeBumps) -> Result<()>{

        self.vault.set_inner(Vault { 
            authority: self.user.key(), 
            balance: 0,
            bump: bump.vault,
        });

        Ok(())
    }
}