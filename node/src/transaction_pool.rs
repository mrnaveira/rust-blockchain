use spec::Transaction;
use std::sync::{Arc, Mutex};

pub type TransactionVec = Vec<Transaction>;

// We don't need to export this type because concurrency is encapsulated in this file
type SyncedTransactionVec = Arc<Mutex<TransactionVec>>;

// Represents a pool of unrealized transactions
// Multiple threads can read/write concurrently to the pool
#[derive(Debug, Clone)]
pub struct TransactionPool {
    transactions: SyncedTransactionVec,
}

// Basic operations in the transaction pool are encapsulated in the implementation
// Encapsulates concurrency concerns, so external callers do not need to know how it's handled
impl TransactionPool {
    // Creates a empty transaction pool
    pub fn new() -> TransactionPool {
        TransactionPool {
            transactions: SyncedTransactionVec::default(),
        }
    }

    pub fn get_transactions(&self) -> TransactionVec {
        let transactions = self.transactions.lock().unwrap();
        transactions.clone()
    }

    pub fn remove_transactions(&self, transactions: Vec<Transaction>) {
        // TODO: transactions should have a nonce to avoid duplicates
        let mut pool_transactions = self.transactions.lock().unwrap();
        pool_transactions.retain(|t| !transactions.contains(t));
    }

    // Adds a new transaction to the pool
    pub fn add_transaction(&self, transaction: Transaction) {
        // TODO: transactions should be validated before being included in the pool
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
        info!("transaction added");
    }
}

#[cfg(test)]
mod tests {
    use spec::Address;

    use super::*;

    #[test]
    fn should_be_empty_after_creation() {
        let pool = TransactionPool::new();
        let transactions = pool.get_transactions();
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_remove_existing_single_transaction() {
        let pool = TransactionPool::new();

        // add a single transaction to the pool...
        let transaction = create_mock_transaction(1);
        pool.add_transaction(transaction.clone());
        assert_eq!(pool.get_transactions().len(), 1);

        // ...and then remove it
        pool.remove_transactions(vec![transaction]);
        assert!(pool.get_transactions().is_empty());
    }

    #[test]
    fn should_remove_existing_multiple_transactions() {
        let pool = TransactionPool::new();

        // add multiple transactions to the pool...
        let tx_1 = create_mock_transaction(1);
        let tx_2 = create_mock_transaction(2);
        let tx_3 = create_mock_transaction(3);
        pool.add_transaction(tx_1.clone());
        pool.add_transaction(tx_2.clone());
        pool.add_transaction(tx_3.clone());
        assert_eq!(pool.get_transactions().len(), 3);

        // and then remove some all but one
        pool.remove_transactions(vec![tx_1, tx_3]);
        assert_eq!(pool.get_transactions().len(), 1);

        // the remaining transaction should be the non-removed one
        let remaining_tx = pool.get_transactions()[0].clone();
        assert_eq!(remaining_tx, tx_2);
    }

    fn create_mock_transaction(amount: u64) -> Transaction {
        Transaction {
            sender: Address::default(),
            recipient: Address::default(),
            amount: amount,
        }
    }
}
