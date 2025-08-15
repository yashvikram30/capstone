use anchor_lang::prelude::*;
use crate::state::{SpendingAccount, MerchantAccount};
use crate::error::AmountError;

pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
    // --- 1. Authorize the Spend ---
    // This logic is identical to our `authorize_spend` instruction.
    let available_to_spend = ctx.accounts.spending_account.spending_limit
        .checked_sub(ctx.accounts.spending_account.amount_spent)
        .unwrap();

    require!(
        amount <= available_to_spend,
        AmountError::SpendingLimitExceeded
    );

    // Update the user's amount spent.
    ctx.accounts.spending_account.amount_spent = ctx.accounts.spending_account.amount_spent
        .checked_add(amount)
        .unwrap();
    
    msg!("Spend of {} authorized.", amount);

    // --- 2. Pay the Merchant ---
    // We now perform a CPI to the System Program to transfer SOL from the user
    // to the merchant's authority wallet.
    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: ctx.accounts.authority.to_account_info(),
        to: ctx.accounts.merchant_authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        cpi_accounts,
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    msg!("Transferred {} lamports to merchant {}.", amount, ctx.accounts.merchant_account.name);

    Ok(())
}

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    // The user making the payment. They must sign and will pay the lamports.
    #[account(mut)]
    pub authority: Signer<'info>,

    // The user's spending account, to verify they have enough limit.
    #[account(
        mut,
        has_one = authority
    )]
    pub spending_account: Account<'info, SpendingAccount>,

    // The merchant's account, to identify the recipient.
    #[account(
        seeds = [b"merchant", merchant_authority.key().as_ref()],
        bump = merchant_account.bump
    )]
    pub merchant_account: Account<'info, MerchantAccount>,

    // The merchant's actual wallet that will receive the payment.
    // The `address` constraint is a powerful security check that ensures this
    // wallet public key matches the `authority` stored in the merchant_account.
    #[account(
        mut,
        address = merchant_account.authority
    )]
    /// CHECK: This is the merchant's wallet, validated by the address constraint.
    pub merchant_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
