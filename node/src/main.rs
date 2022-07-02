#[macro_use]
extern crate log;

mod api;
mod miner;
mod peer;
mod transaction_pool;
mod util;

use api::Api;
use miner::Miner;
use peer::Peer;
use spec::Blockchain;
use transaction_pool::TransactionPool;
use util::{execution, initialize_logger, termination, Config, Context};

fn main() {
    initialize_logger();
    info!("starting up");

    // quit the program when the user inputs Ctrl-C
    termination::set_ctrlc_handler();

    // initialize shared data values
    let config = Config::read();
    let difficulty = config.difficulty;
    let context = Context {
        config,
        blockchain: Blockchain::new(difficulty),
        pool: TransactionPool::new(),
    };

    // initialize the processes
    let miner = Miner::new(&context);
    let api = Api::new(&context);
    let peer = Peer::new(&context);

    // miner, api and peer system run in separate threads
    // because mining is very cpu intensive
    execution::run_in_parallel(vec![&miner, &api, &peer]);
}
