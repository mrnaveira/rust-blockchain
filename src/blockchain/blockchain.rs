use std::sync::{Arc, Mutex};

use crate::blockchain::block::{Block, BlockHash};

pub type BlockVec = Vec<Block>;

// We don't need to export these types because concurrency is encapsulated in this file
type SyncedBlock = Arc<Mutex<Block>>;
type SyncedBlockVec = Arc<Mutex<BlockVec>>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    last_block: SyncedBlock,
    blocks: SyncedBlockVec,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();
        let synced_genesis_block = Arc::new(Mutex::new(genesis_block.clone()));

        let mut blocks = BlockVec::default();
        blocks.push(genesis_block.clone());
        let synced_blocks =  Arc::new(Mutex::new(blocks.clone()));

        let blockchain = Blockchain {
            blocks: synced_blocks.clone(),
            last_block: synced_genesis_block.clone(),
        };

        return blockchain;
    }

    pub fn get_last_block(&self) -> Block {
        let last_block = self.last_block.lock().unwrap();
        return last_block.clone();
    }

    pub fn get_all_blocks(&self) -> BlockVec {
        let blocks = self.blocks.lock().unwrap();
        return blocks.clone();
    }

    pub fn add_block(&self, block: Block) {
        let mut blocks = self.blocks.lock().unwrap();
        let mut last_block = self.last_block.lock().unwrap();

        blocks.push(block.clone());
        *last_block = block.clone();
    }

    fn create_genesis_block() -> Block {
        let index = 0;
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        Block::new(index, nonce, previous_hash, transactions)
    }
}

impl Default for Blockchain {
    fn default() -> Self { Blockchain::new() }
}