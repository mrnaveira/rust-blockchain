#[macro_use]
extern crate log;

mod api;
mod node;
mod peer;
mod transaction_pool;
mod util;

use api::Api;
use node::Node;
use peer::Peer;
use spec::Blockchain;
use transaction_pool::TransactionPool;
use util::{execution, initialize_logger, termination, Config};

fn main() {
    initialize_logger();
    info!("starting up");

    // quit the program when the user inputs Ctrl-C
    termination::set_ctrlc_handler();

    // read the configuration file
    let config = Config::read();

    // initialize the application state
    let blockchain = Blockchain::new(config.difficulty);
    let pool = TransactionPool::new();
    let node = Node::new(blockchain, pool);

    // run the processes in parallel
    let api = Api::new(config.port, &node);
    let peer = Peer::new(&config, &node);
    execution::run_in_parallel(vec![&api, &peer]);
}
