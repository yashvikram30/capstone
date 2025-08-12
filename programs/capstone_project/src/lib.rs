pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8aWorH9nmQ9uQ51Z8Pm33aHdzfnoav7AStPth6yHyKtA");

#[program]
pub mod capstone_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init_vault(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>{
        ctx.accounts.deposit_tokens(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>{
        ctx.accounts.withdraw_sol(amount)?;
        Ok(())
    }

    pub fn initialize_spending_account(ctx: Context<InitializeSpendingAccount>) -> Result<()>{
        ctx.accounts.init_spending_account(&ctx.bumps)?;
        Ok(())
    }

    pub fn update_spending_limit(ctx: Context<UpdateSpendingLimit>) -> Result<()> {
        ctx.accounts.update_spending_limit()?;
        Ok(())
    }

    pub fn authorize_spend(ctx:Context<AuthorizeSpend>, amount: u64) -> Result<()>{
        ctx.accounts.authorize(amount)?;
        Ok(())
    }

    pub fn reset_spend_tracker(ctx:Context<ResetSpendTracker>) -> Result<()> {
        ctx.accounts.reset_spend_tracker()?;
        Ok(())
    }

    pub fn initialize_treasury(ctx:Context<InitializeTreasury>) -> Result<()>{
        ctx.accounts.initialize_treasury(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_yield_account(ctx:Context<InitializeYieldAccount>) -> Result<()>{
        ctx.accounts.initialize_yield_account(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()>{
        ctx.accounts.stake_funds(amount)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        ctx.accounts.unstake_funds(amount)?;
        Ok(())
    }

    pub fn liquidation(ctx: Context<Liquidation>) -> Result<()> {
        ctx.accounts.liquidation()?;
        Ok(())
    }

    pub fn initialize_merchant_account(ctx: Context<InitializeMerchantAccount>, name: String) -> Result<()> {
        ctx.accounts.merchant_account.authority = ctx.accounts.authority.key();
        ctx.accounts.merchant_account.name = name;
        ctx.accounts.merchant_account.bump = ctx.bumps.merchant_account;
        Ok(())       
    }

    pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
        instructions::process_payment::process_payment(ctx, amount)
    }


}
