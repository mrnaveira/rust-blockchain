mod api;
mod blockchain;
mod miner;

use blockchain::{Blockchain, Transaction};
use std::sync::{Arc,Mutex};
use std::thread;

fn main() {
    let blockchain_arc: Arc<Mutex<Blockchain>> = Arc::new(Mutex::new(Blockchain::new()));
    let transaction_pool: Arc<Mutex<Vec<Transaction>>> = Arc::new(Mutex::new(Vec::new()));

    let miner_pool = transaction_pool.clone();
    let miner_blockchain = blockchain_arc.clone();
    thread::spawn(move || {
        miner::run(miner_blockchain, miner_pool);
    });

    let port = 8000;
    api::run(port, blockchain_arc.clone(), transaction_pool.clone())
        .expect("could not start the API");
}