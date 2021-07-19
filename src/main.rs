#[macro_use]
extern crate log;

mod config;
mod logger;
mod api;
mod blockchain;
mod miner;
mod transaction_pool;

use config::Config;
use blockchain::Blockchain;
use miner::MinerSettings;
use transaction_pool::TransactionPool;

fn main() {
    let config = Config::read();
    logger::init();

    info!("starting up");

    // These variables are really "Arc" pointers to a shared memory value
    // So when we clone them, we are only cloning the pointers and not the actual data
    let blockchain = Blockchain::new();
    let transaction_pool = TransactionPool::new();

    // start mining
    let miner_settings = MinerSettings {
        max_nonce: config.max_nonce,
        difficulty: config.difficulty,
        tx_waiting_seconds: config.tx_waiting_seconds
    };
    miner::run(miner_settings, blockchain.clone(), transaction_pool.clone());

    // start the client REST API
    api::run(config.client_port, blockchain.clone(), transaction_pool.clone())
        .expect("could not start the API");
}