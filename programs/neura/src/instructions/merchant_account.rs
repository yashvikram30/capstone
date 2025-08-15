use anchor_lang::prelude::*;
use crate::state::MerchantAccount;

#[derive(Accounts)]
// The name argument from the instruction function is also passed here.
#[instruction(name: String)]
pub struct InitializeMerchantAccount<'info> {
    // The merchant who is creating the account.
    #[account(mut)]
    pub authority: Signer<'info>,

    // The new merchant account PDA.
    // Its space is defined by the constant in the MerchantAccount state.
    #[account(
        init,
        payer = authority,
        space = MerchantAccount::INIT_SPACE,
        seeds = [b"merchant", authority.key().as_ref()],
        bump
    )]
    pub merchant_account: Account<'info, MerchantAccount>,

    pub system_program: Program<'info, System>,
}
