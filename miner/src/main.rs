mod cli;

fn main() {
    let args = cli::parse_args();

    println!("{}", args.miner_address);
    println!("{}", args.node_url);
    println!("{}", args.difficulty);
    println!("{}", args.max_blocks);
    println!("{}", args.max_nonce);

    // read parameters from command line
    // node_url
    // max_blocks
    // max_nonce
    // difficulty
    // tx_waiting_ms
    // miner_address

    // query the base node for the last block
    // query the base node for transactions in the mempool

    // run the miner in a spawn_blocking task
    // await on each mining cycle
    // - check that the mined height is the tip -> submit or ignore
    // - restart mining on the last block
}
