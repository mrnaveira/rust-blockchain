use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: i64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: i64, previous_hash: String, transactions: Vec<Transaction>) -> Block { 
        let block = Block {
            index: index,
            previous_hash: previous_hash,
            timestamp: Utc::now().timestamp_millis(),
            transactions: transactions,
        };

        block
    }
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    pub current_block: Block
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            current_block: genesis_block.clone()
        };

        blockchain.blocks.push(genesis_block.clone());
        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let index = (self.current_block.index + 1) as i64;
        let previous_hash = String::new();
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
}