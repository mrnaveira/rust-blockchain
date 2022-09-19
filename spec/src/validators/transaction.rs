use crate::{types::Transaction, Database};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, PartialEq, Eq, Debug)]
pub enum TransactionError {
    #[error("Sender account does not exist")]
    SenderAccountDoesNotExist,

    #[error("Insufficient funds")]
    InsufficientFunds,
}

pub fn validate_transaction<T: Database>(database: &T, transaction: &Transaction) -> Result<()> {
    let sender_balance = database.get_account_balance(&transaction.sender);

    match sender_balance {
        Some(balance) => {
            // Make sure that the sender has enough funds for the transaction
            if balance < transaction.amount {
                return Err(TransactionError::InsufficientFunds.into());
            }
            Ok(())
        }
        None => Err(TransactionError::SenderAccountDoesNotExist.into()),
    }
}
