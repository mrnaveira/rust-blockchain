mod api;
mod blockchain;
mod miner;
mod transaction_pool;

use blockchain::Blockchain;
use transaction_pool::TransactionPool;

fn main() {
    let blockchain = Blockchain::new();
    let transaction_pool = TransactionPool::new();

    // start mining
    miner::run(blockchain.clone(), transaction_pool.clone());

    // start the REST API
    let port = 8000;
    api::run(port, blockchain.clone(), transaction_pool.clone())
        .expect("could not start the API");
}