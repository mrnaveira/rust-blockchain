use spec::{Block, Blockchain, Transaction};

use crate::{mempool::Mempool, util::Config};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Database {
    blockchain: Blockchain,
    mempool: Mempool,
}

// Only in-memory storage for now
impl Database {
    pub fn new(config: &Config) -> Self {
        let blockchain = Blockchain::new(config.difficulty);
        let mempool = Mempool::default();

        Self {
            blockchain,
            mempool,
        }
    }

    pub fn get_all_blocks(&self) -> Vec<Block> {
        self.blockchain.get_all_blocks()
    }

    pub fn get_last_block(&self) -> Block {
        self.blockchain.get_last_block()
    }

    pub fn add_block(&self, block: Block) -> Result<()> {
        self.blockchain.add_block(block.clone())?;

        // we must remove all submitted transactions present in the new block
        self.mempool.remove_transactions(block.transactions);

        Ok(())
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.mempool.get_transactions()
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        self.mempool.add_transaction(transaction);
    }
}
