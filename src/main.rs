mod api;
mod blockchain;
mod miner;
mod transaction_pool;

use blockchain::SharedBlockchain;
use transaction_pool::TransactionPool;

fn main() {
    let shared_blockchain = SharedBlockchain::default();
    let transaction_pool = TransactionPool::new();

    // start mining
    miner::run(shared_blockchain.clone(), transaction_pool.clone());

    // start the REST API
    let port = 8000;
    api::run(port, shared_blockchain.clone(), transaction_pool.clone())
        .expect("could not start the API");
}