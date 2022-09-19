use anyhow::Result;
use thiserror::Error;

use super::{
    chain::validate_chain, coinbase::validate_coinbase, genesis::validate_genesis,
    proof_of_work::validate_pow, transaction::validate_transaction,
};
use crate::{types::Block, Database};

#[derive(Error, PartialEq, Eq, Debug)]
pub enum BlockError {
    #[error("Invalid hash")]
    InvalidHash,
}

pub fn validate_block<T: Database>(database: &T, block: &Block) -> Result<()> {
    validate_block_metadata(database, block)?;
    validate_block_transactions(database, block)?;

    Ok(())
}

pub fn validate_block_metadata<T: Database>(database: &T, block: &Block) -> Result<()> {
    // consensus rules are different for genesis blocks versus regular blocks
    match block.index {
        0 => validate_genesis(database, block)?,
        _ => validate_chain(database, block)?,
    }

    validate_block_hash(block)?;

    // proof of work validation
    let difficulty = database.get_network().difficulty;
    validate_pow(difficulty, block)?;

    Ok(())
}

pub fn validate_block_hash(block: &Block) -> Result<()> {
    if block.hash != block.calculate_hash() {
        return Err(BlockError::InvalidHash.into());
    }

    Ok(())
}

pub fn validate_block_transactions<T: Database>(database: &T, block: &Block) -> Result<()> {
    let mut transactions = block.transactions.iter();

    // the first transaction is always the coinbase transaction
    // in which the miner receives the mining rewards
    let coinbase = transactions.next();
    validate_coinbase(coinbase)?;

    // all the rest of the transactions are regular ones,
    // where funds get transfered from one account to another
    for transaction in transactions {
        validate_transaction(database, transaction)?;
    }

    Ok(())
}
