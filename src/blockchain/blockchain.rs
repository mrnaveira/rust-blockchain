use anyhow::Result;
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::blockchain::block::{Block, BlockHash};

pub type BlockVec = Vec<Block>;

// We don't need to export this because concurrency is encapsulated in this file
type SyncedBlockVec = Arc<Mutex<BlockVec>>;

// Error types to return when trying to add blocks with invalid fields
#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BlockchainError {
    #[error("Invalid index `{0}`")]
    InvalidIndex(u64),

    #[error("Invalid previous_hash `{0}`")]
    InvalidPreviousHash(BlockHash),

    #[error("Invalid hash `{0}`")]
    InvalidHash(BlockHash),
}

// Struct that holds all the blocks in the blockhain
// Multiple threads can read/write concurrently to the list of blocks
#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: SyncedBlockVec,
}

// Basic operations in the blockchain are encapsulated in the implementation
// Encapsulates concurrency concerns, so external callers do not need to know how it's handled
impl Blockchain {
    // Creates a brand new blockchain with a genesis block
    pub fn new() -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        // add the genesis block to the synced vec of blocks
        let mut blocks = BlockVec::default();
        blocks.push(genesis_block);
        let synced_blocks = Arc::new(Mutex::new(blocks));

        Blockchain {
            blocks: synced_blocks,
        }
    }

    // Returns a copy of the most recent block in the blockchain
    pub fn get_last_block(&self) -> Block {
        let blocks = self.blocks.lock().unwrap();

        blocks[blocks.len() - 1].clone()
    }

    // Returns a copy of the whole list of blocks
    pub fn get_all_blocks(&self) -> BlockVec {
        let blocks = self.blocks.lock().unwrap();

        blocks.clone()
    }

    // Tries to append a new block into the blockchain
    // It will validate that the values of the new block are consistend with the blockchain state
    // This operation is safe to be called concurrently from multiple threads
    pub fn add_block(&self, block: Block) -> Result<()> {
        // the "blocks" attribute is protected by a Mutex
        // so only one thread at a time can access the value when the lock is held
        // that prevents adding multiple valid blocks at the same time
        // preserving the correct order of indexes and hashes of the blockchain
        let mut blocks = self.blocks.lock().unwrap();
        let last = &blocks[blocks.len() - 1];

        // check that the index is valid
        if block.index != last.index + 1 {
            return Err(BlockchainError::InvalidIndex(block.index).into());
        }

        // check that the previous_hash is valid
        if block.previous_hash != last.hash {
            return Err(BlockchainError::InvalidPreviousHash(block.previous_hash).into());
        }

        // check that the hash matches the data
        if block.hash != block.calculate_hash() {
            return Err(BlockchainError::InvalidHash(block.hash).into());
        }

        // append the block to the end
        blocks.push(block);

        Ok(())
    }

    fn create_genesis_block() -> Block {
        let index = 0;
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        Block::new(index, nonce, previous_hash, transactions)
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_valid_genesis_block() {
        let blockchain = Blockchain::new();

        // check that a new blockchain has one and only one block
        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 1);

        // check that the last block is in the blockchain
        let block = blockchain.get_last_block();
        assert_eq!(block.hash, blocks[0].hash);

        // check that the genesis block has valid values
        assert_eq!(block.index, 0);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.previous_hash, BlockHash::default());
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn should_let_adding_valid_blocks() {
        let blockchain = Blockchain::new();

        // create a valid block
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(1, 0, previous_hash, Vec::new());

        // add it to the blockchain and check it was really added
        let result = blockchain.add_block(block.clone());
        assert!(result.is_ok());

        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 2);

        let last_block = blockchain.get_last_block();
        assert_eq!(last_block.hash, block.hash);
    }

    #[test]
    #[should_panic(expected = "Invalid index `2`")]
    fn should_not_let_adding_block_with_invalid_index() {
        let blockchain = Blockchain::new();

        // create a block with invalid index
        let invalid_index = 2;
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(invalid_index, 0, previous_hash, Vec::new());

        // try adding the invalid block, it return an error
        blockchain.add_block(block.clone()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid previous_hash `0`")]
    fn should_not_let_adding_block_with_invalid_previous_hash() {
        let blockchain = Blockchain::new();

        // create a block with invalid previous hash
        let invalid_previous_hash = BlockHash::default();
        let block = Block::new(1, 0, invalid_previous_hash, Vec::new());

        // try adding the invalid block, it return an error
        blockchain.add_block(block.clone()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid hash `0`")]
    fn should_not_let_adding_block_with_invalid_hash() {
        let blockchain = Blockchain::new();

        // create a block with invalid hash
        let previous_hash = blockchain.get_last_block().hash;
        let mut block = Block::new(1, 0, previous_hash, Vec::new());
        block.hash = BlockHash::default();

        // try adding the invalid block, it return an error
        blockchain.add_block(block.clone()).unwrap();
    }
}
