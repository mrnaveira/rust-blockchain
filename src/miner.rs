extern crate async_std;

use async_std::prelude::*;
use async_std::stream;
use async_std::task;
use crate::blockchain::{SharedBlockchain};
use std::time::Duration;
use super::transaction_pool::{SharedTransactionPool};
use std::thread;

const INTERVAL_SECONDS: u64 = 5;

// for now, the mining simply consists on adding a new block every 5 seconds with all the transactions in the pool
async fn mine(shared_blockchain: SharedBlockchain, shared_transaction_pool: SharedTransactionPool) {
    let duration = Duration::from_secs(INTERVAL_SECONDS);
    let mut interval = stream::interval(duration);

    while let Some(_) = interval.next().await {
        let mut blockchain = shared_blockchain.lock().unwrap();  
        let mut transaction_pool = shared_transaction_pool.lock().unwrap();
        
        if !transaction_pool.is_empty() {       
            blockchain.add_block(transaction_pool.clone());
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