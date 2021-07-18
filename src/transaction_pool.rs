use std::sync::{Arc, Mutex};
use crate::blockchain::{Transaction};

pub type TransactionVec = Vec<Transaction>;

// We don't need to export this type because concurrency is encapsulated in this file
type SyncedTransactionVec = Arc<Mutex<TransactionVec>>;

// Represents a pool of unrealized transactions
// Multiple threads can read/write concurrently to the pool
#[derive(Debug, Clone)]
pub struct TransactionPool {
    transactions: SyncedTransactionVec
}

// Basic operations in the transaction pool are encapsulated in the implementation
// Encapsulates concurrency concerns, so external callers do not need to know how it's handled
impl TransactionPool {

    // Creates a empty transaction pool
    pub fn new() -> TransactionPool {
        let pool = TransactionPool {
            transactions: SyncedTransactionVec::default()
        };
        
        return pool;
    }

    // Adds a new transaction to the pool
    pub fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction.clone());
        info!("transaction added");
    }

    // Returns a copy of all transactions and empties the pool
    // This operation is safe to be called concurrently from multiple threads
    pub fn pop(&self) -> TransactionVec {
        // the "transactions" attribute is protected by a Mutex
        // so only one thread at a time can access the value when the lock is held
        // preventing inconsitencies when adding new transactions while a pop is in course
        let mut transactions = self.transactions.lock().unwrap();
        let transactions_clone = transactions.clone();
        transactions.clear();

        return transactions_clone;
    }
}