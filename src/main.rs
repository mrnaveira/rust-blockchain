#[macro_use]
extern crate log;

mod api;
mod config;
mod logger;
mod miner;
mod model;
mod shared_data;

use api::Api;
use config::Config;
use crossbeam_utils::thread;
use miner::Miner;
use model::{Blockchain, TransactionPool};
use shared_data::SharedData;

fn main() {
    logger::init();
    info!("starting up");

    set_ctrlc_handler();

    // initialize shared data values
    let shared_data = SharedData {
        config: Config::read(),
        blockchain: Blockchain::new(),
        pool: TransactionPool::new(),
    };

    // initialize the miner and rest api
    let miner = Miner::new(&shared_data);
    let api = Api::new(&shared_data);

    // run the miner and rest api in separate threads
    thread::scope(|s| {
        s.spawn(move |_| {
            miner.mine().unwrap();
        });
        s.spawn(move |_| {
            api.run().unwrap();
        });
    })
    .unwrap();
}

// Quit the program when the user inputs Ctrl-C
fn set_ctrlc_handler() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
