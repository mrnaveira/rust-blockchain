extern crate async_std;

use async_std::prelude::*;
use async_std::stream;
use async_std::task;
use crate::blockchain::{SharedBlockchain, Blockchain, Block};
use std::time::Duration;
use super::transaction_pool::{SharedTransactionPool, TransactionPool};
use std::thread;

const INTERVAL_SECONDS: u64 = 5;

fn create_next_block(blockchain: Blockchain, transaction_pool: TransactionPool) -> Block {
    let index = (blockchain.current_block.index + 1) as u64;
    let nonce = 0;
    let previous_hash = blockchain.current_block.hash.clone();
    let transactions = transaction_pool.clone();

    Block::new(index, nonce, previous_hash, transactions)
}

async fn mine(shared_blockchain: SharedBlockchain, shared_transaction_pool: SharedTransactionPool) {
    let duration = Duration::from_secs(INTERVAL_SECONDS);
    let mut interval = stream::interval(duration);
    while let Some(_) = interval.next().await {
        let mut blockchain = shared_blockchain.lock().unwrap();  
        let mut transaction_pool = shared_transaction_pool.lock().unwrap();

        if !transaction_pool.is_empty() {      
            let block = create_next_block(blockchain.clone(), transaction_pool.clone());
            blockchain.add_block(block.clone());
            transaction_pool.clear();
        }
    }
}

pub fn run(shared_blockchain: SharedBlockchain, shared_transaction_pool: SharedTransactionPool) {
    let miner_blockchain = shared_blockchain.clone();
    let miner_pool = shared_transaction_pool.clone();

    thread::spawn(move || {
        task::block_on(mine(miner_blockchain, miner_pool))
    });
}