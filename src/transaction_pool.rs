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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_empty_after_creation() {
        let transaction_pool = TransactionPool::new();

        let transactions = transaction_pool.pop();
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_pop_single_value() {
        let transaction_pool = TransactionPool::new();

        // add a new transaction to the pool
        let transaction = create_mock_transaction(1);
        transaction_pool.add_transaction(transaction.clone());

        // pop the values and check that the transaction is included
        let mut transactions = transaction_pool.pop();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].amount, transaction.amount);

        // after the previous pop, the pool should still be empty
        transactions = transaction_pool.pop();
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_pop_multiple_values() {
        let transaction_pool = TransactionPool::new();

        // add a new transaction to the pool
        let transaction_a = create_mock_transaction(1);
        let transaction_b = create_mock_transaction(2);
        transaction_pool.add_transaction(transaction_a.clone());
        transaction_pool.add_transaction(transaction_b.clone());

        // pop the values and check that the transactions are included
        let mut transactions = transaction_pool.pop();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].amount, transaction_a.amount);
        assert_eq!(transactions[1].amount, transaction_b.amount);

        // after the previous pop, the pool should still be empty
        transactions = transaction_pool.pop();
        assert!(transactions.is_empty());
    }

    fn create_mock_transaction(amount: u64) -> Transaction {
        Transaction {
            sender: "1".to_string(),
            recipient: "2".to_string(),
            amount: amount
        }
    }
}    