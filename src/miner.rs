use crate::blockchain::{Blockchain, Block, BlockHash};
use super::transaction_pool::{TransactionVec, TransactionPool};
use std::{thread, time};

pub struct MinerSettings {
    pub max_nonce: u64,
    pub difficulty: usize,
    pub tx_waiting_seconds: u64
}

fn create_next_block(last_block: Block, transactions: TransactionVec, nonce: u64) -> Block {
    let index = (last_block.index + 1) as u64;
    let previous_hash = last_block.hash.clone();

    Block::new(index, nonce, previous_hash, transactions)
}

fn create_target(difficulty: usize) -> BlockHash {
    let target = BlockHash::MAX >> difficulty;

    target
}

fn mine_block(last_block: Block, transactions: TransactionVec, target: BlockHash, max_nonce: u64) -> Option<Block> {
    for nonce in 0..max_nonce {
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

fn mine(settings: MinerSettings, blockchain: Blockchain, transaction_pool: TransactionPool) {
    info!("starting minining with difficulty {}", settings.difficulty);
    let target = create_target(settings.difficulty);
    
    loop { 
        let transactions = transaction_pool.pop();

        // Do not try to mine a block if there are no transactions in the pool
        if transactions.is_empty() {
            sleep_seconds(settings.tx_waiting_seconds);
            continue
        }

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

    thread::spawn(move || {
        mine(settings, miner_blockchain, miner_pool)
    });
}