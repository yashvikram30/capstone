use anchor_lang::prelude::*;

#[error_code]
pub enum AmountError {
    #[msg("Insufficient funds for withdrawal.")]
    InsufficientFunds,
    #[msg("The requested spend amount exceeds the remaining limit.")]
    SpendingLimitExceeded,
}
