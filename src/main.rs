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
use transaction_pool::TransactionPool;

fn main() {
    let config = Config::read();
    logger::init();

    info!("starting up");

    let blockchain = Blockchain::new();
    let transaction_pool = TransactionPool::new();

    // start mining
    miner::run(blockchain.clone(), transaction_pool.clone());

    // start the client REST API
    api::run(config.client_port, blockchain.clone(), transaction_pool.clone())
        .expect("could not start the API");
}