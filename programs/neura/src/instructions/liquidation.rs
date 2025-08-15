use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{SpendingAccount, Treasury, Vault, YieldAccount};

const LIQUIDATION_THRESHOLD: u128 = 120; // If ratio is below 120%, liquidation can occur.
const TARGET_COLLATERAL_RATIO: u128 = 150;

#[derive(Accounts)]
pub struct Liquidation <'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"spending", authority.key().as_ref()],
        bump
    )]
    pub spending_account: Account<'info, SpendingAccount>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"yield", authority.key().as_ref()],
        bump = yield_account.bump
    )]
    pub yield_account: Account<'info, YieldAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury", authority.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    
}

impl <'info> Liquidation <'info> {
    
    pub fn liquidation(&mut self) -> Result<()>{

        let amount_spent = self.spending_account.amount_spent;
        let staked_amount = self.yield_account.staked_amount;
        let price_update = &self.price_update;

        let maximum_age: u64 = 60;
        let feed_id: [u8; 32] = get_feed_id_from_hex("0xef0d8b614545449452e9f8d4623e34ade2ba2ac67362100e27457bf6fc8894c4")?;

        let current_price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

        let staked_price_in_usd_cents = ((staked_amount as u128).checked_mul(current_price.price as u128).unwrap() as u128).checked_div((10u128).pow(15)).unwrap();
        let amount_spent_in_cents = amount_spent as u128;

        let collateral_ratio = if amount_spent_in_cents > 0 {
            staked_price_in_usd_cents.checked_mul(100).unwrap().checked_div(amount_spent_in_cents).unwrap()
        } else{
            0
        };

        let pyth_price = current_price.price as u128;
        let pyth_expo = current_price.exponent.abs() as u32;
        let staked_lamports = self.yield_account.staked_amount as u128;

        // --- 2. Check if Liquidation is Necessary ---
        if collateral_ratio < LIQUIDATION_THRESHOLD {
            msg!("Ratio is below threshold. Starting liquidation...");

            // --- 3. Calculate How Much to Liquidate ---
            // We want to find the new, smaller debt amount that makes the ratio 150%.
            // Formula: NewDebt = (Collateral * 100) / TargetRatio
            let target_debt_in_cents = staked_price_in_usd_cents
                .checked_mul(100).unwrap()
                .checked_div(TARGET_COLLATERAL_RATIO).unwrap();

            // The amount to liquidate is the difference between the old debt and the new target debt.
            let debt_to_repay_in_cents = amount_spent_in_cents
                .checked_sub(target_debt_in_cents).unwrap();

            // Convert the cents to repay back into lamports of SOL to sell.
            // Formula: Lamports = (Cents * 10^9 * 10^expo) / (Price * 100)
            let lamports_to_liquidate = debt_to_repay_in_cents
                .checked_mul((10u128).pow(9 + pyth_expo)).unwrap()
                .checked_div(pyth_price).unwrap()
                .checked_div(100).unwrap();

            // --- 4. Perform the Liquidation (Transfer and State Update) ---
            msg!("Liquidating {} lamports to repay ${:.2} of debt.", lamports_to_liquidate, debt_to_repay_in_cents as f64 / 100.0);

            // Transfer lamports from Treasury to user's Vault. This simulates the "sale".
            // Debit from the treasury.
            **self.treasury.to_account_info().try_borrow_mut_lamports()? -= lamports_to_liquidate as u64;
            // Credit to the vault.
            **self.vault.to_account_info().try_borrow_mut_lamports()? += lamports_to_liquidate as u64;

            // Update the state accounts
            self.yield_account.staked_amount -= lamports_to_liquidate as u64;
            self.spending_account.amount_spent -= debt_to_repay_in_cents as u64;

            msg!("Liquidation successful. New ratio will be >= 150%.");

        } else {
            msg!("Position is healthy. No liquidation needed.");
        }

        Ok(())
    
    }
}
