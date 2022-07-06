use spec::{Block, BlockHash, Transaction};

use crate::cli::MinerArgs;

pub fn mine_block(
    args: &MinerArgs,
    target: BlockHash,
    last_block: Block,
    transactions: Vec<Transaction>,
) -> Option<Block> {
    for nonce in 0..args.max_nonce {
        let next_block = create_next_block(&last_block, transactions.clone(), nonce);

        // A valid block must have a hash with enough starting zeroes
        // To check that, we simply compare against a binary data mask
        if next_block.hash < target {
            return Some(next_block);
        }
    }

    None
}

// Creates a valid next block for a blockchain
// Takes into account the index and the hash of the previous block
pub fn create_next_block(last_block: &Block, transactions: Vec<Transaction>, nonce: u64) -> Block {
    let index = (last_block.index + 1) as u64;
    let previous_hash = last_block.hash;

    // the hash of the new block is automatically calculated on creation
    Block::new(index, nonce, previous_hash, transactions)
}
