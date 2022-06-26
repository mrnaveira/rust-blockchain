mod address;
mod block;
mod blockchain;
mod transaction;
mod transaction_pool;

// Explicitly controlling which individual identifiers we export
// It also avoids verbose module imports from other files
pub use address::Address;
pub use block::{Block, BlockHash};
pub use blockchain::{Blockchain, BlockchainError, BLOCK_SUBSIDY};
pub use transaction::Transaction;
pub use transaction_pool::{TransactionPool, TransactionVec};

#[cfg(test)]
pub use address::test_util;
