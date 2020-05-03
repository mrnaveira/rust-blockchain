use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Serialize};
use serde_json;

use crate::blockchain::transaction::Transaction;

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Block { 
        let mut block = Block {
            index: index,
            timestamp: Utc::now().timestamp_millis(),
            previous_hash: previous_hash,
            hash: String::new(),
            transactions: transactions,
        };
        block.hash = block.calculate_hash();

        block
    }

    pub fn calculate_hash(&self) -> String {
        // the "hash" field could be setted, so to be consistent we need to make sure we don't use it
        let mut hashable_data = self.clone();
        hashable_data.hash = String::new();

        let serialized = serde_json::to_string(&hashable_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(&serialized);

        hasher.result_str()
    }
}