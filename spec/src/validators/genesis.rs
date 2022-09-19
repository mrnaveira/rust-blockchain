use anyhow::Result;
use thiserror::Error;

use crate::types::hash::ConsensusHashable;
use crate::types::Block;
use crate::Database;

#[derive(Error, PartialEq, Eq, Debug)]
pub enum GenesisError {
    #[error("Invalid index")]
    InvalidIndex,

    #[error("Duplicated genesis")]
    DuplicatedGenesis,

    #[error("Mismatched network")]
    MismatchedNetwork,
}

pub fn validate_genesis<T: Database>(database: &T, block: &Block) -> Result<()> {
    // a genesis block must always be the first block
    if block.index != 0 {
        return Err(GenesisError::InvalidIndex.into());
    }

    // make sure there is not a genesis block already present
    let genesis_already_exists = database.get_tip_block().is_some();
    if genesis_already_exists {
        return Err(GenesisError::DuplicatedGenesis.into());
    }

    // the previous_hash of the genesis block MUST be the hash of the network definition
    // that way we can differentiate between any number of different networks
    // beacuse the genesis block for each one will be different
    let network_hash = database.get_network().consensus_hash();
    if block.previous_hash != network_hash {
        return Err(GenesisError::MismatchedNetwork.into());
    }

    Ok(())
}
