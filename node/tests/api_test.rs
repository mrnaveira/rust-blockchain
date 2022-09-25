mod utils;
use rusty_fork::rusty_fork_test;
use serial_test::serial;
use spec::types::hash::ConsensusHash;
use spec::types::Block;
use spec::types::Transaction;
use spec::validators::BLOCK_SUBSIDY;

use crate::utils::alice;
use crate::utils::miner_address;
use crate::utils::Miner;

use crate::utils::RestApi;
use crate::utils::TestServerBuilder;

// We run each test in a separated process to force resource liberation (i.e. network ports)
rusty_fork_test! {

#[test]
#[serial]
fn test_should_get_a_valid_genesis_block() {
    // start the node
    let node = TestServerBuilder::new().build();
    node.start();

    // mine the genesis block
    let miner = Miner::new();
    miner.mine_blocks(1);

    // list the blocks by querying the REST API
    let blocks = node.get_blocks();

    // check that the blocks only contain the genesis block
    assert_eq!(blocks.len(), 1);
    let genesis_block = blocks.first().unwrap();
    assert_eq!(genesis_block.index, 0);
}

#[test]
#[serial]
fn test_should_let_add_transactions() {
    // start the node
    let node = TestServerBuilder::new().build();
    node.start();

    // mine the genesis block
    let miner = Miner::new();
    miner.mine_blocks(1);
    let genesis_block = node.get_last_block();

    // create and add a new transaction to the pool
    // the sender must the mining address,
    // as it should have funds from the coinbase reward of the genesis block
    let transaction = Transaction {
        sender: miner_address(),
        recipient: alice(),
        amount: 10 as u64,
    };
    let res = node.add_transaction(&transaction);
    assert_eq!(res.status().as_u16(), 200);

    // wait for the transaction to be mined
    let miner = Miner::new();
    miner.mine_blocks(1);

    // check that a new bock was added...
    let blocks = node.get_blocks();
    assert_eq!(blocks.len(), 2);
    let mined_block = blocks.last().unwrap();

    // ...and is valid
    assert_eq!(mined_block.index, 1);
    assert_eq!(mined_block.previous_hash, genesis_block.hash);

    // ...and contains the transaction that we added (plus the coinbase)
    assert_eq!(mined_block.transactions.len(), 2);
    let mined_transaction = mined_block.transactions.last().unwrap();
    assert_eq!(*mined_transaction, transaction);
}

#[test]
#[serial]
fn test_should_let_add_valid_block() {
    // start the node
    let node = TestServerBuilder::new().build();
    node.start();

    // mine the genesis block
    let miner = Miner::new();
    miner.mine_blocks(1);
    let genesis_block = node.get_last_block();

    // build a valid block
    let coinbase = Transaction {
        sender: miner_address(),
        recipient: alice(),
        amount: BLOCK_SUBSIDY,
    };
    let valid_block = Block::new(1, 0, genesis_block.hash, vec![coinbase]);

    // the node should accept the block
    let res = node.add_block(&valid_block);
    assert_eq!(res.status().as_u16(), 200);
}

#[test]
#[serial]
fn test_should_not_let_add_invalid_block() {
    // start the node
    let node = TestServerBuilder::new().build();
    node.start();

    // mine the genesis block
    let miner = Miner::new();
    miner.mine_blocks(1);

    // build a block with an invalid previous hash
    let invalid_block = Block::new(1, 0, ConsensusHash::default(), vec![]);

    // the node should return an error when adding the block
    let res = node.add_block(&invalid_block);
    assert_eq!(res.status().as_u16(), 400);
}
}
