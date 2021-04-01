use crate::blockchain::{Blockchain, Block, BlockHash};
use super::transaction_pool::{TransactionVec, TransactionPool};
use std::{thread, time};

const MAX_NONCE: u64 = 1_000_000;
const DIFFICULTY: usize = 10;
const TRANSACTION_WAITING_SECONDS: u64 = 5;

fn create_next_block(last_block: Block, transactions: TransactionVec, nonce: u64) -> Block {
    let index = (last_block.index + 1) as u64;
    let previous_hash = last_block.hash.clone();

    Block::new(index, nonce, previous_hash, transactions)
}

fn create_target(difficulty: usize) -> BlockHash {
    let target = BlockHash::MAX >> difficulty;

    target
}

fn mine_block(last_block: Block, transactions: TransactionVec, target: BlockHash) -> Option<Block> {
    for nonce in 0..MAX_NONCE {
        let next_block = create_next_block(last_block.clone(), transactions.clone(), nonce);
 
        if next_block.hash < target {
            return Some(next_block);
        }
    }

    None
}

fn sleep_seconds(seconds: u64) {
    let wait_duration = time::Duration::from_secs(seconds);
    thread::sleep(wait_duration);
}

fn mine(blockchain: Blockchain, transaction_pool: TransactionPool) {
    let target = create_target(DIFFICULTY);
    
    // TODO: add a parameter to start and stop mining
    loop { 
        let transactions = transaction_pool.pop();

        // Do not try to mine a block if there are no transactions in the pool
        if transactions.is_empty() {
            sleep_seconds(TRANSACTION_WAITING_SECONDS);
            continue
        }

        let last_block = blockchain.get_last_block();
        let mining_result = mine_block(last_block, transactions.clone(), target.clone());
        match mining_result {
            Some(block) => {
                blockchain.add_block(block.clone());
            }
            None => {
                // TODO: raise exception when a valid block was not found
                println!("No valid block was found");
            }
        }
    }
}

pub fn run(blockchain: Blockchain, transaction_pool: TransactionPool) {
    let miner_blockchain = blockchain.clone();
    let miner_pool = transaction_pool.clone();

    thread::spawn(move || {
        mine(miner_blockchain, miner_pool)
    });
}