use spec::{types::Block, validators::validate_pow};

use crate::cli::MinerArgs;

pub fn mine_block(args: &MinerArgs, block_template: &Block) -> Option<Block> {
    let mut block_canditate = block_template.clone();

    // mining is just trying different nonces until the block hash has enough starting zeroes
    for nonce in 0..args.max_nonce {
        block_canditate.nonce = nonce;
        block_canditate.hash = block_canditate.calculate_hash();

        if validate_pow(args.difficulty, &block_canditate).is_ok() {
            return Some(block_canditate);
        }
    }

    None
}
