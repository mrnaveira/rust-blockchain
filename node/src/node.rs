use spec::{Block, Blockchain, Transaction};

use crate::transaction_pool::TransactionPool;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Node {
    blockchain: Blockchain,
    pool: TransactionPool,
}

impl Node {
    pub fn new(blockchain: Blockchain, pool: TransactionPool) -> Self {
        Self { blockchain, pool }
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
        self.pool.remove_transactions(block.transactions);

        Ok(())
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.pool.get_transactions()
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        self.pool.add_transaction(transaction);
    }
}
