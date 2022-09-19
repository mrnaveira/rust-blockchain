use crate::{types::Block, Database};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, PartialEq, Eq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ChainError {
    #[error("Invalid index")]
    InvalidIndex,

    #[error("Blockchain is empty")]
    BlockchainIsEmpty,

    #[error("Invalid previous_hash")]
    InvalidPreviousHash,
}

pub fn validate_chain<T: Database>(database: &T, block: &Block) -> Result<()> {
    // we assume that that genesis block is present
    // as the sequence validation does not make sense on the genesis block
    let tip_block = match database.get_tip_block() {
        Some(value) => value,
        None => return Err(ChainError::BlockchainIsEmpty.into()),
    };

    // check that the index is valid
    if block.index != tip_block.index + 1 {
        return Err(ChainError::InvalidIndex.into());
    }

    // check that the previous_hash is valid
    if block.previous_hash != tip_block.hash {
        return Err(ChainError::InvalidPreviousHash.into());
    }

    Ok(())
}
