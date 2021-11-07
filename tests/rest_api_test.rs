mod common;

use serial_test::serial;

use crate::common::{
    api_utils::{add_block, add_transaction, get_blocks, Block, BlockHash, Transaction},
    server_utils::{run_in_server_instance, wait_for_mining},
};

#[test]
#[serial]
#[cfg(unix)]
fn test_should_get_a_valid_genesis_block() {
    run_in_server_instance(|| {
        // list the blocks by querying the REST API
        let blocks = get_blocks();

        // check that the blocks only contain the genesis block
        assert_eq!(blocks.len(), 1);
        let genesis_block = blocks.first().unwrap();

        // check that the genesis block fields are valid
        assert_eq!(genesis_block.index, 0);
        assert_eq!(genesis_block.nonce, 0);
        assert_eq!(genesis_block.previous_hash, BlockHash::default());
        assert!(genesis_block.transactions.is_empty());
    });
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_let_add_transactions() {
    run_in_server_instance(|| {
        let blocks = get_blocks();
        let genesis_block = blocks.first().unwrap();

        // create and add a new transaction to the pool
        let transaction = Transaction {
            sender: "1".to_string(),
            recipient: "2".to_string(),
            amount: 100 as u64,
        };
        let res = add_transaction(&transaction);
        assert_eq!(res.status().as_u16(), 200);

        // wait for the transaction to be mined
        wait_for_mining();

        // check that a new bock was added...
        let blocks = get_blocks();
        assert_eq!(blocks.len(), 2);
        let mined_block = blocks.last().unwrap();

        // ...and it contains the transaction we added
        assert_eq!(mined_block.index, 1);
        assert_eq!(mined_block.previous_hash, genesis_block.hash);
        assert_eq!(mined_block.transactions.len(), 1);

        let mined_transaction = mined_block.transactions.first().unwrap();
        assert_eq!(*mined_transaction, transaction);
    });
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_let_add_valid_block() {
    run_in_server_instance(|| {
        let blocks = get_blocks();
        let genesis_block = blocks.first().unwrap();

        let valid_block = Block {
            // there is the genesis block already, so the next index is 1
            index: 1,
            timestamp: 0,
            nonce: 0,
            // the previous hash is checked
            previous_hash: genesis_block.hash,
            // the api automatically recalculates the hash...
            // ...so no need to add a valid one here
            hash: BlockHash::default(),
            transactions: [].to_vec(),
        };
        let res = add_block(&valid_block);
        assert_eq!(res.status().as_u16(), 200);
    });
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_not_let_add_invalid_block() {
    run_in_server_instance(|| {
        // let's try to add a new INVALID block, should return an error
        let invalid_block = Block {
            index: 0, // not valid index, the genesis block already has index 0
            timestamp: 0,
            nonce: 0,
            previous_hash: BlockHash::default(), // also not valid
            hash: BlockHash::default(),
            transactions: [].to_vec(),
        };
        let res = add_block(&invalid_block);
        assert_eq!(res.status().as_u16(), 400);
    });
}
