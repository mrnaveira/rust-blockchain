use anyhow::Result;
use thiserror::Error;

use crate::types::Transaction;

// The reward for successfully mining a block
// For now, this amount is constant
pub const BLOCK_SUBSIDY: u64 = 100;

#[derive(Error, PartialEq, Eq, Debug)]
pub enum CoinbaseError {
    #[error("Coinbase transaction not found")]
    CoinbaseTransactionNotFound,

    #[error("Invalid coinbase amount")]
    InvalidCoinbaseAmount,
}

pub fn validate_coinbase(coinbase: Option<&Transaction>) -> Result<()> {
    // The coinbase transaction is required in a valid block
    let coinbase = match coinbase {
        Some(transaction) => transaction,
        None => return Err(CoinbaseError::CoinbaseTransactionNotFound.into()),
    };

    // In coinbase transactions, we only need to check that the amount is valid,
    // because whoever provides a valid proof-of-work block can receive the new coins
    // i.e. the sender is totally ignored and its balance never decreased
    let is_valid_amount = coinbase.amount == BLOCK_SUBSIDY;
    if !is_valid_amount {
        return Err(CoinbaseError::InvalidCoinbaseAmount.into());
    }

    Ok(())
}
