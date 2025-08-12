use anchor_lang::{accounts::program, prelude::*, system_program::{transfer, Transfer}};

use crate::Vault;


#[derive(Accounts)]
pub struct Deposit<'info>{

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info,Vault>,

    pub system_program: Program<'info, System>
}

impl <'info> Deposit <'info> {
    
    pub fn deposit_tokens(&mut self, amount: u64) -> Result<()>{

        let program = self.system_program.to_account_info();
        let accounts = Transfer{
            from:self.authority.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(program, accounts);

        transfer(ctx, amount)?;

        self.vault.reload()?;
        self.vault.balance = self.vault.balance.checked_add(amount).unwrap();


        Ok(())
    }
}