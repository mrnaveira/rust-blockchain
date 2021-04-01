use std::sync::{Arc, Mutex};
use crate::blockchain::{Transaction};

pub type TransactionVec = Vec<Transaction>;

// We don't need to export this type because concurrency is encapsulated in this file
type SyncedTransactionVec = Arc<Mutex<TransactionVec>>;

#[derive(Debug, Clone)]
pub struct TransactionPool {
    transactions: SyncedTransactionVec
}

impl TransactionPool {
    pub fn new() -> TransactionPool {
        let pool = TransactionPool {
            transactions: SyncedTransactionVec::default()
        };
        return pool;
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction.clone());
    }

    pub fn pop(&self) -> TransactionVec {
        let mut transactions = self.transactions.lock().unwrap();
        let transactions_clone = transactions.clone();
        transactions.clear();

        return transactions_clone;
    }
}