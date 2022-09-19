use anyhow::Result;
use thiserror::Error;

use crate::types::Block;

#[derive(Error, PartialEq, Eq, Debug)]
pub enum ProofOfWorkError {
    #[error("Invalid difficulty")]
    InvalidDifficulty,
}

pub fn validate_pow(difficulty: u32, block: &Block) -> Result<()> {
    if block.hash.leading_zeros() < difficulty {
        return Err(ProofOfWorkError::InvalidDifficulty.into());
    }

    Ok(())
}
