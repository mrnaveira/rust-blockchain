use crate::{
    model::{Block, BlockHash, Blockchain, TransactionPool, TransactionVec},
    util::{execution::Runnable, Context},
};
use anyhow::Result;
use std::{thread, time};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("No valid block was mined at index `{0}`")]
    BlockNotMined(u64),
}

#[derive(Debug, Clone)]
pub struct Miner {
    max_blocks: u64,
    max_nonce: u64,
    difficulty: usize,
    tx_waiting_ms: u64,
    blockchain: Blockchain,
    pool: TransactionPool,
}

impl Runnable for Miner {
    fn run(&self) -> Result<()> {
        self.mine()
    }
}

impl Miner {
    pub fn new(context: &Context) -> Miner {
        Miner {
            max_blocks: context.config.max_blocks,
            max_nonce: context.config.max_nonce,
            difficulty: context.config.difficulty,
            tx_waiting_ms: context.config.tx_waiting_ms,
            blockchain: context.blockchain.clone(),
            pool: context.pool.clone(),
        }
    }

    // Try to constanly calculate and append new valid blocks to the blockchain,
    // including all pending transactions in the transaction pool each time
    pub fn mine(&self) -> Result<()> {
        info!("starting minining with difficulty {}", self.difficulty);
        let target = self.create_target(self.difficulty);

        // In each loop it tries to find the next valid block and append it to the blockchain
        let mut block_counter = 0;
        loop {
            if self.must_stop_mining(block_counter) {
                info!("block limit reached, stopping mining");
                return Ok(());
            }

            // Empty all transactions from the pool, they will be included in the new block
            let transactions = self.pool.pop();

            // Do not try to mine a block if there are no transactions in the pool
            if transactions.is_empty() {
                self.sleep_millis(self.tx_waiting_ms);
                continue;
            }

            // try to find a valid next block of the blockchain
            let last_block = self.blockchain.get_last_block();
            let mining_result =
                self.mine_block(&last_block, transactions.clone(), target, self.max_nonce);
            match mining_result {
                Some(block) => {
                    info!("valid block found for index {}", block.index);
                    self.blockchain.add_block(block.clone())?;
                    block_counter += 1;
                }
                None => {
                    let index = last_block.index + 1;
                    error!("no valid block was foun for index {}", index);
                    return Err(MinerError::BlockNotMined(index).into());
                }
            }
        }
    }

    // Creates binary data mask with the amount of left padding zeroes indicated by the "difficulty" value
    // Used to easily compare if a newly created block has a hash that matches the difficulty
    fn create_target(&self, difficulty: usize) -> BlockHash {
        BlockHash::MAX >> difficulty
    }

    // check if we have hit the limit of mined blocks (if the limit is set)
    fn must_stop_mining(&self, block_counter: u64) -> bool {
        self.max_blocks > 0 && block_counter >= self.max_blocks
    }

    // Suspend the execution of the thread by a particular amount of milliseconds
    fn sleep_millis(&self, millis: u64) {
        let wait_duration = time::Duration::from_millis(millis);
        thread::sleep(wait_duration);
    }

    // Tries to find the next valid block of the blockchain
    // It will create blocks with different "nonce" values until one has a hash that matches the difficulty
    // Returns either a valid block (that satisfies the difficulty) or "None" if no block was found
    fn mine_block(
        &self,
        last_block: &Block,
        transactions: TransactionVec,
        target: BlockHash,
        max_nonce: u64,
    ) -> Option<Block> {
        for nonce in 0..max_nonce {
            let next_block = self.create_next_block(last_block, transactions.clone(), nonce);

            // A valid block must have a hash with enough starting zeroes
            // To check that, we simply compare against a binary data mask
            if next_block.hash < target {
                return Some(next_block);
            }
        }

        None
    }

    // Creates a valid next block for a blockchain
    // Takes into account the index and the hash of the previous block
    fn create_next_block(
        &self,
        last_block: &Block,
        transactions: TransactionVec,
        nonce: u64,
    ) -> Block {
        let index = (last_block.index + 1) as u64;
        let previous_hash = last_block.hash;

        // hash of the new block is automatically calculated on creation
        Block::new(index, nonce, previous_hash, transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Transaction;

    // We use SHA 256 hashes
    const MAX_DIFFICULTY: usize = 256;

    #[test]
    fn test_create_next_block() {
        let miner = create_miner();
        let block = create_empty_block();

        let next_block = miner.create_next_block(&block, Vec::new(), 0);

        // the next block must follow the previous one
        assert_eq!(next_block.index, block.index + 1);
        assert_eq!(next_block.previous_hash, block.hash);
    }

    #[test]
    fn test_create_target_valid_difficulty() {
        let miner = create_miner();

        // try all possibilities of valid difficulties
        // the target must have as many leading zeroes
        for difficulty in 0..MAX_DIFFICULTY {
            let target = miner.create_target(difficulty);
            assert_eq!(target.leading_zeros(), difficulty as u32);
        }
    }

    #[test]
    fn test_create_target_overflowing_difficulty() {
        let miner = create_miner();

        // when passing an overflowing difficulty,
        // it must default to the max difficulty
        let target = miner.create_target(MAX_DIFFICULTY + 1);
        assert_eq!(target.leading_zeros(), MAX_DIFFICULTY as u32);
    }

    #[test]
    fn test_mine_block_found() {
        let miner = create_miner();
        let last_block = create_empty_block();

        // let's use a small difficulty target for fast testing
        let difficulty = 1;
        let target = miner.create_target(difficulty);

        // this should be more than enough nonces to find a block with only 1 zero
        let max_nonce = 1_000;

        // check that the block is mined
        let result = miner.mine_block(&last_block, Vec::new(), target, max_nonce);
        assert!(result.is_some());

        // check that the block is valid
        let mined_block = result.unwrap();
        assert_mined_block_is_valid(&mined_block, &last_block, difficulty);
    }

    #[test]
    fn test_mine_block_not_found() {
        let miner = create_miner();
        let last_block = create_empty_block();

        // let's use a high difficulty target to never find a block
        let difficulty = MAX_DIFFICULTY;
        let target = miner.create_target(difficulty);

        // with a max_nonce so low, we will never find a block
        // and also the test will end fast
        let max_nonce = 10;

        // check that the block is not mined
        let result = miner.mine_block(&last_block, Vec::new(), target, max_nonce);
        assert!(result.is_none());
    }

    #[test]
    fn test_mine_successful() {
        // with a max_nonce so hight and difficulty so low
        // we will always find a valid block
        let mut miner = create_miner();
        miner.max_nonce = 1_000_000;
        miner.difficulty = 1;

        let blockchain = miner.blockchain.clone();
        let pool = miner.pool.clone();

        add_mock_transaction(&pool);
        let result = miner.mine();

        // mining should be successful
        assert!(result.is_ok());

        // a new block should have been added to the blockchain
        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 2);
        let genesis_block = &blocks[0];
        let mined_block = &blocks[1];

        // the mined block must be valid
        assert_mined_block_is_valid(mined_block, genesis_block, miner.difficulty);

        // the mined block must include the transaction added previously
        let mined_transactions = &mined_block.transactions;
        assert_eq!(mined_transactions.len(), 1);

        // the transaction pool must be empty
        // because the transaction was added to the block when mining
        let transactions = pool.pop();
        assert!(transactions.is_empty());
    }

    #[test]
    #[should_panic(expected = "No valid block was mined at index `1`")]
    fn test_mine_not_found() {
        // with a max_nonce so low and difficulty so high
        // we will never find a valid block
        let mut miner = create_miner();
        miner.max_nonce = 1;
        miner.difficulty = MAX_DIFFICULTY;

        let pool = &miner.pool;
        add_mock_transaction(pool);

        // mining should return a BlockNotMined error
        miner.mine().unwrap();
    }

    fn create_miner() -> Miner {
        let max_blocks = 1;
        let max_nonce = 1;
        let difficulty = 1;
        let tx_waiting_ms = 1;

        let blockchain = Blockchain::new();
        let pool = TransactionPool::new();

        return Miner {
            max_blocks,
            max_nonce,
            difficulty,
            tx_waiting_ms,
            blockchain,
            pool,
        };
    }

    fn create_empty_block() -> Block {
        return Block::new(0, 0, BlockHash::default(), Vec::new());
    }

    fn add_mock_transaction(pool: &TransactionPool) {
        let transaction = Transaction {
            sender: "1".to_string(),
            recipient: "2".to_string(),
            amount: 3,
        };
        pool.add_transaction(transaction.clone());
    }

    fn assert_mined_block_is_valid(mined_block: &Block, previous_block: &Block, difficulty: usize) {
        assert_eq!(mined_block.index, previous_block.index + 1);
        assert_eq!(mined_block.previous_hash, previous_block.hash);
        assert!(mined_block.hash.leading_zeros() >= difficulty as u32);
    }
}
