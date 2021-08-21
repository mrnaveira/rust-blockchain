#[macro_use]
extern crate log;

mod config;
mod logger;
mod api;
mod blockchain;
mod miner;
mod transaction_pool;

use api::Api;
use config::Config;
use blockchain::Blockchain;
use miner::Miner;
use transaction_pool::TransactionPool;
use std::thread;

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
    pool: &TransactionPool
) -> std::thread::JoinHandle<()> {
    let miner = Miner::new(
        config.max_blocks,
        config.max_nonce,
        config.difficulty,
        config.tx_waiting_ms,
        &blockchain,
        &pool,
    );

    let handler = thread::spawn(move || {
        miner.mine().unwrap();
    });

    return handler;
}

// starts the api in a separate thread and returns the handler
fn create_api_thread(
    config: &Config,
    blockchain: &Blockchain,
    pool: &TransactionPool
) -> std::thread::JoinHandle<()> {
    let api = Api::new(
        config.port,
        &blockchain,
        &pool,
    );

    let handler = thread::spawn(move || {
        api.run().unwrap();
    });

    return handler;
}

// wait for all children threads to finish
fn blocking_wait(handles: Vec<std::thread::JoinHandle<()>>) {
    for handle in handles {
        handle.join().unwrap();
    }
}