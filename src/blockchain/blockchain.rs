use serde::{Serialize};

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

const TRANSACTION_POOL_SIZE: i32 = 3;

#[derive(Debug, Serialize, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub current_block: Block,
    pub transaction_pool: Vec<Transaction>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            current_block: genesis_block.clone(),
            transaction_pool: Vec::new()
        };

        blockchain.blocks.push(genesis_block.clone());
        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let index = (self.current_block.index + 1) as u64;
        let previous_hash = self.current_block.hash.clone();
        let block = Block::new(index, previous_hash, transactions);

        self.blocks.push(block.clone());
        self.current_block = block.clone();
        block.clone()
    }

    fn create_genesis_block() -> Block {
        let index = 0;
        let previous_hash = String::new();
        let transactions = Vec::new();

        Block::new(index, previous_hash, transactions)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transaction_pool.push(transaction);
        
        // for now, we are going to auto create a new block on every 3 transactions
        let pool_len = self.transaction_pool.len();
        if pool_len as i32 >= TRANSACTION_POOL_SIZE {
            self.add_block(self.transaction_pool.clone());
            self.transaction_pool.clear();
        }
    }
}