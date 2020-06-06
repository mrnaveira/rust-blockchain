extern crate async_std;

use async_std::prelude::*;
use async_std::stream;
use async_std::task;

use crate::blockchain::{Blockchain, Transaction};

use std::sync::{Arc, Mutex};
use std::time::Duration;

const INTERVAL_SECONDS: u64 = 5;

struct MinerState {
    blockchain_arc: Arc<Mutex<Blockchain>>,
    transaction_pool_arc: Arc<Mutex<Vec<Transaction>>>
}

// for now, the mining simply consists on adding a new block every 5 seconds with all the transactions in the pool
async fn mine(state: MinerState) {
    let blockchain_arc = &state.blockchain_arc;
    let transaction_pool_arc = &state.transaction_pool_arc;
    let duration = Duration::from_secs(INTERVAL_SECONDS);
    let mut interval = stream::interval(duration);

    while let Some(_) = interval.next().await {
        let mut blockchain = blockchain_arc.lock().unwrap();  
        let mut transaction_pool = transaction_pool_arc.lock().unwrap();
        
        if !transaction_pool.is_empty() {       
            blockchain.add_block(transaction_pool.clone());
            transaction_pool.clear();
        }
    }
}

pub fn run(blockchain_arc: Arc<Mutex<Blockchain>>, transaction_pool_arc: Arc<Mutex<Vec<Transaction>>>) {
    let miner_state = MinerState {
        blockchain_arc: blockchain_arc,
        transaction_pool_arc: transaction_pool_arc
    };

    task::block_on(mine(miner_state))
}