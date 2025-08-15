use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct YieldAccount {
    
    pub authority: Pubkey,
    pub staked_amount: u64,
    pub bump: u8,
}
