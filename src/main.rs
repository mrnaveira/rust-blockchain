#[macro_use]
extern crate log;

mod api;
mod miner;
mod model;
mod util;

use api::Api;
use miner::Miner;
use model::{Blockchain, TransactionPool};
use util::{execution, initialize_logger, termination, Config, Context};

fn main() {
    initialize_logger();
    info!("starting up");

    // quit the program when the user inputs Ctrl-C
    termination::set_ctrlc_handler();

    // initialize shared data values
    let context = Context {
        config: Config::read(),
        blockchain: Blockchain::new(),
        pool: TransactionPool::new(),
    };

    // initialize the miner and rest api
    let miner = Miner::new(&context);
    let api = Api::new(&context);

    // miner and api run in separate threads
    // because mining is very cpu intensive
    execution::run_in_parallel(vec![&miner, &api]);
}
