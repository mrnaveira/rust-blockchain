mod block;
// In this case, we want "blockchain" to be
// the name of the parent module that includes
// a "blockchain" file as well
#[allow(clippy::module_inception)]
mod blockchain;
mod transaction;

// Explicitly controlling which individual identifiers we export
// It also avoids verbose module imports from other files
pub use block::Block;
pub use block::BlockHash;
pub use blockchain::Blockchain;
pub use blockchain::BlockchainError;
pub use transaction::Transaction;
