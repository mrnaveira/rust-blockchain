use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Serialize, Deserialize};
use serde_json;

const TRANSACTION_POOL_SIZE: i32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Block { 
        let mut block = Block {
            index: index,
            timestamp: Utc::now().timestamp_millis(),
            previous_hash: previous_hash,
            hash: String::new(),
            transactions: transactions,
        };
        block.hash = block.calculate_hash();

        block
    }

    pub fn calculate_hash(&self) -> String {
        // the "hash" field could be setted, so to be consistent we need to make sure we don't use it
        let mut hashable_data = self.clone();
        hashable_data.hash = String::new();

        let serialized = serde_json::to_string(&hashable_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(&serialized);

        hasher.result_str()
    }
}

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