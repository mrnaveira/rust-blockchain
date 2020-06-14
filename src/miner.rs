use crate::blockchain::{SharedBlockchain, Blockchain, Block, BlockHash};
use super::transaction_pool::{SharedTransactionPool, TransactionPool};
use std::{thread, time};

const MAX_NONCE: u64 = 1_000_000;
const DIFFICULTY: usize = 10;
const WAIT_FOR_TRANSACTIONS_IN_SECS: u64 = 5;

fn create_next_block(blockchain: Blockchain, transaction_pool: TransactionPool, nonce: u64) -> Block {
    let index = (blockchain.current_block.index + 1) as u64;
    let previous_hash = blockchain.current_block.hash.clone();
    let transactions = transaction_pool.clone();

    Block::new(index, nonce, previous_hash, transactions)
}

fn create_target(difficulty: usize) -> BlockHash {
    let target = BlockHash::MAX >> difficulty;

    target
}

fn get_blockhain_contents(shared_blockchain: SharedBlockchain) -> Blockchain {
    let blockchain = shared_blockchain.lock().unwrap();
    return blockchain.clone();
}

fn pop_transaction_pool(shared_transaction_pool: SharedTransactionPool) -> TransactionPool {
    let mut transaction_pool = shared_transaction_pool.lock().unwrap();
    let transactions = transaction_pool.clone();
    transaction_pool.clear();
    return transactions;
}

fn mine_block(blockchain: Blockchain, transaction_pool: TransactionPool, target: BlockHash) -> Option<Block> {
    for nonce in 0..MAX_NONCE {
        let block = create_next_block(blockchain.clone(), transaction_pool.clone(), nonce);
 
        if block.hash < target {
            return Some(block);
        }
    }

    None
}

fn mine(shared_blockchain: SharedBlockchain, shared_transaction_pool: SharedTransactionPool) {
    let target = create_target(DIFFICULTY);
    
    // TODO: add a parameter to start and stop mining
    loop { 
        let blockchain = get_blockhain_contents(shared_blockchain.clone());
        let transactions = pop_transaction_pool(shared_transaction_pool.clone());

        // Do not try to mine a block if there are no transactions in the pool
        if transactions.is_empty() {
            let wait_duration = time::Duration::from_secs(WAIT_FOR_TRANSACTIONS_IN_SECS);
            thread::sleep(wait_duration);
            continue
        }

        let mining_result = mine_block(blockchain.clone(), transactions.clone(), target.clone());
        match mining_result {
            Some(block) => {
                let mut blockchain = shared_blockchain.lock().unwrap();  
                blockchain.add_block(block.clone());
            }
            None => {
                // TODO: raise exception when a valid block was not found
                println!("No valid block was found");
            }
        }
    }
}

pub fn run(shared_blockchain: SharedBlockchain, shared_transaction_pool: SharedTransactionPool) {
    let miner_blockchain = shared_blockchain.clone();
    let miner_pool = shared_transaction_pool.clone();

    thread::spawn(move || {
        mine(miner_blockchain, miner_pool)
    });
}