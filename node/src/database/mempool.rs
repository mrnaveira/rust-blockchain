use spec::types::Transaction;

// Represents a pool of unrealized transactions
#[derive(Debug, Clone, Default)]
pub struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    // Add a new transaction to the pool
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        info!("transaction added");
    }

    pub fn remove_transactions(&mut self, transactions: &[Transaction]) {
        // TODO: transactions should have a nonce to avoid duplicates
        self.transactions.retain(|t| !transactions.contains(t));
    }
}

#[cfg(test)]
mod tests {
    use spec::types::Address;

    use super::*;

    #[test]
    fn should_be_empty_after_creation() {
        let mempool = Mempool::default();
        let transactions = mempool.get_transactions();
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_remove_existing_single_transaction() {
        let mut mempool = Mempool::default();

        // add a single transaction to the pool...
        let transaction = create_mock_transaction(1);
        mempool.add_transaction(transaction.clone());
        assert_eq!(mempool.get_transactions().len(), 1);

        // ...and then remove it
        mempool.remove_transactions(&vec![transaction]);
        assert!(mempool.get_transactions().is_empty());
    }

    #[test]
    fn should_remove_existing_multiple_transactions() {
        let mut mempool = Mempool::default();

        // add multiple transactions to the pool...
        let tx_1 = create_mock_transaction(1);
        let tx_2 = create_mock_transaction(2);
        let tx_3 = create_mock_transaction(3);
        mempool.add_transaction(tx_1.clone());
        mempool.add_transaction(tx_2.clone());
        mempool.add_transaction(tx_3.clone());
        assert_eq!(mempool.get_transactions().len(), 3);

        // and then remove some all but one
        mempool.remove_transactions(&vec![tx_1, tx_3]);
        assert_eq!(mempool.get_transactions().len(), 1);

        // the remaining transaction should be the non-removed one
        let remaining_tx = mempool.get_transactions()[0].clone();
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
