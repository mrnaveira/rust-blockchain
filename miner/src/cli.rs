use clap::Parser;
use spec::types::Address;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct MinerArgs {
    /// Account address that will receive the mining rewards
    #[clap(short = 'a', long, value_parser)]
    pub miner_address: Address,

    /// Network address of the node
    #[clap(
        short = 'n',
        long,
        value_parser,
        default_value = "http://localhost:8000"
    )]
    pub node_url: String,

    /// Minimum number of starting zeroes needed in a block hash for a proof-of-work valid block
    #[clap(short = 'd', long, value_parser, default_value = "10")]
    pub difficulty: u32,

    /// Maximum number of blocks to mine (0 for unlimited)
    #[clap(long, value_parser, default_value = "0")]
    pub max_blocks: u64,

    /// Maximum nonce that will be used when mining a block
    #[clap(long, value_parser, default_value = "1000000")]
    pub max_nonce: u64,
}

pub fn parse_args() -> MinerArgs {
    MinerArgs::parse()
}
