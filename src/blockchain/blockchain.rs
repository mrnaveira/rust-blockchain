use serde::{Serialize};
use std::sync::{Arc, Mutex};

use crate::blockchain::block::{Block, BlockHash};
use crate::blockchain::transaction::Transaction;

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

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block.clone());
        self.current_block = block.clone();
    }

    fn create_genesis_block() -> Block {
        let index = 0;
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        Block::new(index, nonce, previous_hash, transactions)
    }
}

pub type SharedBlockchain = Arc<Mutex<Blockchain>>;

pub fn create_shared_blockchain() -> SharedBlockchain {
    return Arc::new(Mutex::new(Blockchain::new()));
}