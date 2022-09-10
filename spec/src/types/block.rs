use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    hash::{ConsensusHash, ConsensusHashable},
    Transaction,
};

// Represents a block in a blockchain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub previous_hash: ConsensusHash,
    pub hash: ConsensusHash,
    pub transactions: Vec<Transaction>,
}

impl Block {
    // Create a brand new block. The hash value will be caclulated and set automatically.
    pub fn new(
        index: u64,
        nonce: u64,
        previous_hash: ConsensusHash,
        transactions: Vec<Transaction>,
    ) -> Block {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp_millis(),
            nonce,
            previous_hash,
            hash: ConsensusHash::default(),
            transactions,
        };
        block.hash = block.calculate_hash();

        block
    }

    // Calculate the hash value of the block
    pub fn calculate_hash(&self) -> ConsensusHash {
        // We cannot use the hash field to calculate the hash
        // so we zeroed it out
        let mut hashable_data = self.clone();
        hashable_data.hash = ConsensusHash::default();

        hashable_data.consensus_hash()
    }
}
