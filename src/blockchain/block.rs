use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use ethereum_types::U256;
use serde::Serialize;

use crate::blockchain::transaction::Transaction;

// We encapsulate the paricular hash value implementation
// to be able to easily change it in the future
pub type BlockHash = U256;

// Represents a block in a blockchain
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

impl Block {
    // Create a brand new block. The hash value will be caclulated and set automatically.
    pub fn new(
        index: u64,
        nonce: u64,
        previous_hash: BlockHash,
        transactions: Vec<Transaction>,
    ) -> Block {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp_millis(),
            nonce,
            previous_hash,
            hash: BlockHash::default(),
            transactions,
        };
        block.hash = block.calculate_hash();

        block
    }

    // Calculate the hash value of the block
    pub fn calculate_hash(&self) -> BlockHash {
        // We cannot use the hash field to calculate the hash
        let mut hashable_data = self.clone();
        hashable_data.hash = BlockHash::default();
        let serialized = serde_json::to_string(&hashable_data).unwrap();

        // Cacluate and return the SHA-256 hash value for the block
        let mut byte_hash = <[u8; 32]>::default();
        let mut hasher = Sha256::new();

        hasher.input_str(&serialized);
        hasher.result(&mut byte_hash);

        U256::from(byte_hash)
    }
}
