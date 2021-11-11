use anyhow::Result;
use std::sync::{Arc, Mutex};
use thiserror::Error;

use super::{Block, BlockHash};

pub type BlockVec = Vec<Block>;

// We don't need to export this because concurrency is encapsulated in this file
type SyncedBlockVec = Arc<Mutex<BlockVec>>;

// Error types to return when trying to add blocks with invalid fields
#[derive(Error, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BlockchainError {
    #[error("Invalid index")]
    InvalidIndex,

    #[error("Invalid previous_hash")]
    InvalidPreviousHash,

    #[error("Invalid hash")]
    InvalidHash,

    #[error("Invalid difficulty")]
    InvalidDifficulty,
}

// Struct that holds all the blocks in the blockhain
// Multiple threads can read/write concurrently to the list of blocks
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub difficulty: u32,
    blocks: SyncedBlockVec,
}

// Basic operations in the blockchain are encapsulated in the implementation
// Encapsulates concurrency concerns, so external callers do not need to know how it's handled
impl Blockchain {
    // Creates a brand new blockchain with a genesis block
    pub fn new(difficulty: u32) -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        // add the genesis block to the synced vec of blocks
        let mut blocks = BlockVec::default();
        blocks.push(genesis_block);
        let synced_blocks = Arc::new(Mutex::new(blocks));

        Blockchain {
            difficulty,
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
            return Err(BlockchainError::InvalidIndex.into());
        }

        // check that the previous_hash is valid
        if block.previous_hash != last.hash {
            return Err(BlockchainError::InvalidPreviousHash.into());
        }

        // check that the hash matches the data
        if block.hash != block.calculate_hash() {
            return Err(BlockchainError::InvalidHash.into());
        }

        // check that the difficulty is correct
        if block.hash.leading_zeros() < self.difficulty {
            return Err(BlockchainError::InvalidDifficulty.into());
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

        let mut block = Block::new(index, nonce, previous_hash, transactions);

        // to easily sync multiple nodes in a network, the genesis blocks must match
        // so we clear the timestamp so the hash of the genesis block is predictable
        block.timestamp = 0;
        block.hash = block.calculate_hash();

        block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_DIFFICULTY: u32 = 0;

    #[test]
    fn should_have_valid_genesis_block() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

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
        let blockchain = Blockchain::new(NO_DIFFICULTY);

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
    fn should_not_let_adding_block_with_invalid_index() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with invalid index
        let invalid_index = 2;
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(invalid_index, 0, previous_hash, Vec::new());

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidIndex);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_previous_hash() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with invalid previous hash
        let invalid_previous_hash = BlockHash::default();
        let block = Block::new(1, 0, invalid_previous_hash, Vec::new());

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidPreviousHash);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_hash() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with invalid hash
        let previous_hash = blockchain.get_last_block().hash;
        let mut block = Block::new(1, 0, previous_hash, Vec::new());
        block.hash = BlockHash::default();

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidHash);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_difficulty() {
        // set up a blockchain with an insane difficulty
        let difficulty: u32 = 30;
        let blockchain = Blockchain::new(difficulty);

        // create a valid block
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(1, 0, previous_hash, Vec::new());

        // ensure that the hash actually does NOT meet the difficulty
        assert!(block.hash.leading_zeros() < difficulty);

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidDifficulty);
    }

    fn assert_err(result: Result<(), anyhow::Error>, error_type: BlockchainError) {
        let err = result.unwrap_err().downcast::<BlockchainError>().unwrap();
        assert_eq!(err, error_type);
    }
}
