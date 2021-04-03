#[macro_use]
extern crate log;

mod api;
mod blockchain;
mod miner;
mod transaction_pool;

use log::LevelFilter;
use env_logger::{Builder, Target};

use blockchain::Blockchain;
use transaction_pool::TransactionPool;

fn init_logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter(None, LevelFilter::Info);
    builder.init();
}

fn main() {
    init_logger();
    info!("starting up");

    let blockchain = Blockchain::new();
    let transaction_pool = TransactionPool::new();

    // start mining
    miner::run(blockchain.clone(), transaction_pool.clone());

    // start the REST API
    let port = 8000;
    api::run(port, blockchain.clone(), transaction_pool.clone())
        .expect("could not start the API");
}