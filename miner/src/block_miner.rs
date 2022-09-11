use spec::{
    types::{Block, Transaction},
    validators::validate_pow,
};

use crate::cli::MinerArgs;

pub fn mine_block(
    args: &MinerArgs,
    last_block: Block,
    transactions: Vec<Transaction>,
) -> Option<Block> {
    // mining is just trying different nonces until the block hash has enough starting zeroes
    for nonce in 0..args.max_nonce {
        let next_block = create_next_block(&last_block, transactions.clone(), nonce);

        if validate_pow(args.difficulty, &next_block).is_ok() {
            return Some(next_block);
        }
    }

    None
}

// Creates a valid next block for a blockchain
// Takes into account the index and the hash of the previous block
pub fn create_next_block(last_block: &Block, transactions: Vec<Transaction>, nonce: u64) -> Block {
    let index = (last_block.index + 1) as u64;
    let previous_hash = last_block.hash.clone();

    // the hash of the new block is automatically calculated on creation
    Block::new(index, nonce, previous_hash, transactions)
}
