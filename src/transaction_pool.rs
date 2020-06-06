use std::sync::{Arc, Mutex};

use crate::blockchain::{Transaction};

pub type SharedTransactionPool = Arc<Mutex<Vec<Transaction>>>;

pub fn create_shared_transaction_pool() -> SharedTransactionPool {
    return Arc::new(Mutex::new(Vec::new()));
}