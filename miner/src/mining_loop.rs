use core::time;

use spec::{Address, BlockHash, Transaction, BLOCK_SUBSIDY};

use crate::{block_miner::mine_block, cli::MinerArgs, node_client::NodeClient};

pub fn run_mining_loop(args: MinerArgs, node_client: impl NodeClient) {
    let mut blocks_mined: u64 = 0;
    let target = create_target(args.difficulty);

    while should_keep_mining(blocks_mined, &args) {
        let last_block = node_client.get_last_block();

        // Do not try to mine a block if there are no transactions in the pool
        let mut transactions = wait_for_transactions(&args, &node_client);

        // Add the coinbase transaction as the first transaction in the block
        let coinbase = create_coinbase_transaction(args.miner_address.clone());
        transactions.insert(0, coinbase);

        // Try to mine the new block
        let mining_result = mine_block(&args, target, last_block, transactions);
        match mining_result {
            Some(new_block) => {
                // TODO: properly log the mined block
                println!("Block mined");
                // TODO: handle the error in block submission
                // TODO: wait for the block to be included in the node
                node_client.submit_block(&new_block);
                blocks_mined += 1;
            }
            None => {
                // TODO: log or return an error
                // println!("Block not found");
            }
        }
    }
}

// Creates binary data mask with the amount of left padding zeroes indicated by the "difficulty" value
// Used to easily compare if a newly created block has a hash that matches the difficulty
pub fn create_target(difficulty: u32) -> BlockHash {
    BlockHash::MAX >> difficulty
}

pub fn create_coinbase_transaction(miner_address: Address) -> Transaction {
    Transaction {
        sender: Address::default(),
        recipient: miner_address,
        amount: BLOCK_SUBSIDY,
    }
}

fn wait_for_transactions(args: &MinerArgs, node_client: &impl NodeClient) -> Vec<Transaction> {
    loop {
        let transactions = node_client.get_mempool_transactions();

        // finally, we found some transactions in the pool!
        if !transactions.is_empty() {
            return transactions;
        }

        // no transactions yet, let's keep waiting
        println!(
            "No transactions found, checking again in {} milliseconds",
            args.tx_waiting_ms
        );
        sleep_millis(args.tx_waiting_ms);
    }
}

fn should_keep_mining(blocks_mined: u64, args: &MinerArgs) -> bool {
    if args.max_blocks == 0 {
        return true;
    }

    blocks_mined == args.max_blocks
}

// Suspend the execution of the thread by a particular amount of milliseconds
pub fn sleep_millis(millis: u64) {
    let wait_duration = time::Duration::from_millis(millis);
    std::thread::sleep(wait_duration);
}
