mod api;
mod blockchain;
mod miner;
mod transaction_pool;

use blockchain::{create_shared_blockchain, SharedBlockchain};
use transaction_pool::{create_shared_transaction_pool, SharedTransactionPool};

fn main() {
    let shared_blockchain: SharedBlockchain = create_shared_blockchain();
    let shared_transaction_pool: SharedTransactionPool = create_shared_transaction_pool();

    // start mining
    miner::run(shared_blockchain.clone(), shared_transaction_pool.clone());

    // start the REST API
    let port = 8000;
    api::run(port, shared_blockchain.clone(), shared_transaction_pool.clone())
        .expect("could not start the API");
}