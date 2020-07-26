use std::sync::{Arc, Mutex};
use crate::blockchain::{Transaction};

pub type TransactionPool = Vec<Transaction>;

pub type SharedTransactionPool = Arc<Mutex<TransactionPool>>;