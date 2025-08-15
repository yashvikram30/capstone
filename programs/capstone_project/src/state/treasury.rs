use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub authority: Pubkey,
    pub bump: u8,
}