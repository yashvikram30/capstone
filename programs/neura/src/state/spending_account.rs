use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SpendingAccount {
    pub authority: Pubkey,
    pub spending_limit: u64,
    pub amount_spent: u64,
    pub bump: u8,
}
