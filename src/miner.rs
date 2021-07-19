use crate::blockchain::{Blockchain, Block, BlockHash};
use super::transaction_pool::{TransactionVec, TransactionPool};
use std::{thread, time};

// Wrapper of mining settings
// Used to group together related values when calling the miner
pub struct MinerSettings {
    pub max_nonce: u64,
    pub difficulty: usize,
    pub tx_waiting_seconds: u64
}

// Creates a valid next block for a blockchain
// Takes into account the index and the hash of the previous block
fn create_next_block(last_block: &Block, transactions: TransactionVec, nonce: u64) -> Block {
    let index = (last_block.index + 1) as u64;
    let previous_hash = last_block.hash.clone();

    // hash of the new block is automatically calculated on creation
    Block::new(index, nonce, previous_hash, transactions)
}

// Creates binary data mask with the amount of left padding zeroes indicated by the "difficulty" value
// Used to easily compare if a newly created block has a hash that matches the difficulty
fn create_target(difficulty: usize) -> BlockHash {
    let target = BlockHash::MAX >> difficulty;

    target
}

// Tries to find the next valid block of the blockchain
// It will create blocks with different "nonce" values until one has a hash that matches the difficulty
// Returns either a valid block (that satisfies the difficulty) or "None" if no block was found
fn mine_block(last_block: Block, transactions: TransactionVec, target: BlockHash, max_nonce: u64) -> Option<Block> {
    for nonce in 0..max_nonce {
        let next_block = create_next_block(&last_block, transactions.clone(), nonce);
 
        // A valid block must have a hash with enough starting zeroes
        // To check that, we simply compare against a binary data mask
        if next_block.hash < target {
            return Some(next_block);
        }
    }

    None
}

// Suspend the execution of the thread by a particular amount of seconds
fn sleep_seconds(seconds: u64) {
    let wait_duration = time::Duration::from_secs(seconds);
    thread::sleep(wait_duration);
}

// Try to constanly calculate and append new valid blocks to the blockchain,
// including all pending transactions in the transaction pool each time
fn mine(settings: MinerSettings, blockchain: Blockchain, transaction_pool: TransactionPool) {
    info!("starting minining with difficulty {}", settings.difficulty);
    let target = create_target(settings.difficulty);
    
    // For now, mining is running forever until the applications stops
    // In each loop it tries to find the next valid block and append it to the blockchain
    loop {
        // Empty all transactions from the pool, they will be included in the new block
        let transactions = transaction_pool.pop();

        // Do not try to mine a block if there are no transactions in the pool
        if transactions.is_empty() {
            sleep_seconds(settings.tx_waiting_seconds);
            continue
        }

        // try to find a valid next block of the blockchain
        let last_block = blockchain.get_last_block();
        let mining_result = mine_block(last_block, transactions.clone(), target.clone(), settings.max_nonce);
        match mining_result {
            Some(block) => {
                info!("valid block found for index {}", block.index);
                blockchain.add_block(block.clone());
            }
            None => {
                // TODO: raise exception when a valid block was not found
                error!("no valid block was found");
            }
        }
    }
}

pub fn run(settings: MinerSettings, blockchain: Blockchain, transaction_pool: TransactionPool) {
    let miner_blockchain = blockchain.clone();
    let miner_pool = transaction_pool.clone();

    // Mining is done on a separate thread to allow concurrent operations (i.e. API)
    thread::spawn(move || {
        mine(settings, miner_blockchain, miner_pool)
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    // We use SHA 256 hashes
    const MAX_DIFFICULTY: usize = 256;

    #[test]
    fn test_create_next_block() {
        let block = Block::new(0, 0, BlockHash::default(), Vec::new());
        let next_block = create_next_block(&block, Vec::new(), 0);

        // the next block must follow the previous one
        assert_eq!(next_block.index, block.index + 1);
        assert_eq!(next_block.previous_hash, block.hash);
    }

    #[test]
    fn test_create_target_valid_difficulty() {
        // try all possibilities of valid difficulties
        // the target must have as many leading zeroes
        for difficulty in 0..MAX_DIFFICULTY {
            let target = create_target(difficulty);
            assert_eq!(target.leading_zeros(), difficulty as u32);
        }  
    }

    #[test]
    fn test_create_target_overflowing_difficulty() {
        // when passing an overflowing difficulty,
        // it must default to the max difficulty
        let target = create_target(MAX_DIFFICULTY + 1);
        assert_eq!(target.leading_zeros(), MAX_DIFFICULTY as u32); 
    }

}