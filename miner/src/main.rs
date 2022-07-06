use miner::{cli, mining_loop::run_mining_loop, node_client::NetworkNodeClient};

fn main() {
    let args = cli::parse_args();
    let node_url = args.node_url.clone();
    let node_client = NetworkNodeClient::new(node_url);

    run_mining_loop(args, node_client);
}
