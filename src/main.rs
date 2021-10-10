#[macro_use]
extern crate log;

mod api;
mod blockchain;
mod config;
mod logger;
mod miner;
mod transaction_pool;

use api::Api;
use blockchain::Blockchain;
use config::Config;
use miner::Miner;
use std::thread;
use transaction_pool::TransactionPool;

fn main() {
    logger::init();
    info!("starting up");

    set_ctrlc_handler();

    // initialize shared data values
    let config = Config::read();
    let blockchain = Blockchain::new();
    let pool = TransactionPool::new();

    // hold the children thread handles
    let mut handles = Vec::new();

    // start mining in a separate thread
    let miner_handle = create_miner_thread(&config, &blockchain, &pool);
    handles.push(miner_handle);

    // start the REST API in a separate thread
    let api_handle = create_api_thread(&config, &blockchain, &pool);
    handles.push(api_handle);

    // wait for all children threads to finish
    blocking_wait(handles);
}

// Quit the program when the user inputs Ctrl-C
fn set_ctrlc_handler() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

// starts mining in a separate thread and returns the handler
fn create_miner_thread(
    config: &Config,
    blockchain: &Blockchain,
    pool: &TransactionPool,
) -> std::thread::JoinHandle<()> {
    let miner = Miner::new(
        config.max_blocks,
        config.max_nonce,
        config.difficulty,
        config.tx_waiting_ms,
        blockchain,
        pool,
    );

    thread::spawn(move || {
        miner.mine().unwrap();
    })
}

// starts the api in a separate thread and returns the handler
fn create_api_thread(
    config: &Config,
    blockchain: &Blockchain,
    pool: &TransactionPool,
) -> std::thread::JoinHandle<()> {
    let api = Api::new(config.port, blockchain, pool);

    thread::spawn(move || {
        api.run().unwrap();
    })
}

// wait for all children threads to finish
fn blocking_wait(handles: Vec<std::thread::JoinHandle<()>>) {
    for handle in handles {
        handle.join().unwrap();
    }
}
