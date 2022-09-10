pub mod balance;
pub mod block_hash;
pub mod coinbase;
pub mod genesis;
pub mod proof_of_work;
pub mod sequence;

use crate::{
    types::{Block, Network},
    Database,
};
use anyhow::Result;

use self::{
    balance::validate_balance, block_hash::validate_block_hash, coinbase::validate_coinbase,
    genesis::validate_genesis, proof_of_work::validate_pow, sequence::validate_sequence,
};

/* 
pub struct Consensus<T: Database> {
    pub network: Network,
    database: T,
}

impl<T: Database> Consensus<T> {
    pub fn new(network: Network, database: T) -> Self {
        Self { network, database }
    }

    pub fn get_all_blocks(&self) -> Vec<Block> {
        self.database.get_all_blocks()
    }

    pub fn get_tip_block(&self) -> Option<Block> {
        self.database.get_tip_block()
    }

    pub fn append_block(&mut self, block: &Block) -> Result<()> {
        self.validate_tip_block(block)?;
        self.process_transactions(block)?;
        self.database.append_block(block);

        // submitted transactions should be removed from the mempool
        self.database.remove_from_mempool(&block.transactions);

        Ok(())
    }

    pub fn validate_tip_block(&self, block: &Block) -> Result<()> {
        match block.index {
            0 => validate_genesis(&self.database, block)?,
            _ => validate_sequence(&self.database, block)?,
        }

        validate_block_hash(block)?;
        validate_pow(self.network.difficulty, block)?;

        self.validate_transactions(block)?;

        Ok(())
    }

    pub fn validate_transactions(&self, block: &Block) -> Result<()> {
        let mut transactions = block.transactions.iter();

        // the first transaction is always the coinbase transaction
        // in which the miner receives the mining rewards
        let coinbase = transactions.next();
        validate_coinbase(coinbase)?;

        // all the rest of the transactions are regular ones,
        // where funds get transfered from one account to another
        for transaction in transactions {
            validate_balance(&self.database, transaction)?;
        }

        Ok(())
    }

    pub fn process_transactions(&mut self, block: &Block) -> Result<()> {
        // make sure that the transactions in the block are valid
        self.validate_transactions(block)?;

        let mut transactions = block.transactions.iter();

        // after validation, we know that the coinbase exists and it's valid
        let coinbase = transactions.next().unwrap();
        self.database
            .add_funds(&coinbase.recipient, coinbase.amount);

        // same as before, at this point we know that all the regular transactions are valid
        for transaction in transactions {
            self.database.transfer(
                &transaction.sender,
                &transaction.recipient,
                transaction.amount,
            )?;
        }

        Ok(())
    }
}
*/