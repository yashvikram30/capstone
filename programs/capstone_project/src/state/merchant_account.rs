use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MerchantAccount {
    // The public key of the merchant who owns this account.
    pub authority: Pubkey,
    // The display name of the merchant.
    #[max_len(200)]
    pub name: String,
    // The PDA bump seed.
    pub bump: u8,
}

