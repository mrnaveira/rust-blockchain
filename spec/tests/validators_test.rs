mod util;

use spec::types::hash::{ConsensusHash, ConsensusHashable};
use spec::types::{Block, Network, Transaction};
use spec::validators::{
    validate_block, BlockError, ChainError, CoinbaseError, ProofOfWorkError, TransactionError,
    BLOCK_SUBSIDY,
};
use spec::Database;
use util::MockDatabase;

use crate::util::alice;
use crate::util::build_coinbase_transaction;
use crate::util::miner_address;

#[test]
fn should_accept_valid_blocks() {
    let mut db = MockDatabase::default();

    // add a valid genesis block
    db.append_genesis_block().unwrap();
    let genesis = db.get_tip_block().unwrap();

    // now let's create a valid block after the genesis

    // create a valid coinbase transaction
    let coinbase = build_coinbase_transaction();
    let mut block_transactions = vec![coinbase];

    // add a regular transaction
    // note that the miner address have funds from the genesis block's coinbase
    let transactions = vec![Transaction::new(miner_address(), alice(), 10)];
    block_transactions.append(&mut transactions.clone());
    let block = Block::new(genesis.index + 1, 0, genesis.hash, block_transactions);

    // the validator should accept the new block
    validate_block(&db, &block).unwrap();
}

#[test]
fn should_reject_blocks_with_invalid_index() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // create a block with invalid index
    let coinbase = build_coinbase_transaction();
    let tip_block = db.get_tip_block().unwrap();
    let block = Block::new(tip_block.index + 2, 0, tip_block.hash, vec![coinbase]);

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<ChainError>().unwrap();
    assert!(matches!(inner_err, ChainError::InvalidIndex));
}

#[test]
fn should_reject_blocks_with_invalid_previous_hash() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // create a block with invalid previous hash
    let coinbase = build_coinbase_transaction();
    let tip_block = db.get_tip_block().unwrap();
    let block = Block::new(
        tip_block.index + 1,
        0,
        ConsensusHash::default(),
        vec![coinbase],
    );

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<ChainError>().unwrap();
    assert!(matches!(inner_err, ChainError::InvalidPreviousHash));
}

#[test]
fn should_reject_blocks_with_invalid_hash() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // create a block with invalid previous hash
    let coinbase = build_coinbase_transaction();
    let tip_block = db.get_tip_block().unwrap();
    let mut block = Block::new(tip_block.index + 1, 0, tip_block.hash, vec![coinbase]);
    block.hash = ConsensusHash::default();

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<BlockError>().unwrap();
    assert!(matches!(inner_err, BlockError::InvalidHash));
}

#[test]
fn should_reject_blocks_with_invalid_difficulty() {
    // set up a blockchain with an insane difficulty
    let network = Network {
        description: "Test network".to_string(),
        difficulty: 30,
        timestamp: 0,
    };
    let db = MockDatabase::new(network.clone());

    // build the genesis block
    let coinbase = build_coinbase_transaction();
    let block = Block::new(0, 0, network.consensus_hash(), vec![coinbase]);

    // ensure that the hash actually does NOT meet the difficulty
    assert!(block.hash.leading_zeros() < network.difficulty);

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<ProofOfWorkError>().unwrap();
    assert!(matches!(inner_err, ProofOfWorkError::InvalidDifficulty));
}

#[test]
fn should_reject_blocks_with_no_coinbase() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // create a block without a coinbase
    let tip_block = db.get_tip_block().unwrap();
    let block = Block::new(tip_block.index + 1, 0, tip_block.hash, vec![]);

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<CoinbaseError>().unwrap();
    assert!(matches!(
        inner_err,
        CoinbaseError::CoinbaseTransactionNotFound
    ));
}

#[test]
fn should_reject_blocks_with_invalid_coinbase() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // create a block with an invalid coinbase amount
    let mut coinbase = build_coinbase_transaction();
    coinbase.amount += 1;
    let tip_block = db.get_tip_block().unwrap();
    let block = Block::new(tip_block.index + 1, 0, tip_block.hash, vec![coinbase]);

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<CoinbaseError>().unwrap();
    assert!(matches!(inner_err, CoinbaseError::InvalidCoinbaseAmount));
}

#[test]
fn should_reject_transactions_with_insufficient_funds() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // let's create a transaction with an invalid amount
    // at this point the miner address has funds from the genesis coinbase
    // but we are going to try sending a bigger amount
    let invalid_transaction = Transaction::new(miner_address(), alice(), BLOCK_SUBSIDY + 1);

    // create a block with the invalid transaction
    let coinbase = build_coinbase_transaction();
    let tip_block = db.get_tip_block().unwrap();
    let block = Block::new(
        tip_block.index + 1,
        0,
        tip_block.hash,
        vec![coinbase, invalid_transaction],
    );

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<TransactionError>().unwrap();
    assert!(matches!(inner_err, TransactionError::InsufficientFunds));
}

#[test]
fn should_reject_transactions_with_non_existent_sender() {
    let mut db = MockDatabase::default();
    db.append_genesis_block().unwrap();

    // let's create a transaction with a sender account (alice) that does not exist yet
    let invalid_transaction = Transaction::new(alice(), miner_address(), 1);

    // create a block with the invalid transaction
    let coinbase = build_coinbase_transaction();
    let tip_block = db.get_tip_block().unwrap();
    let block = Block::new(
        tip_block.index + 1,
        0,
        tip_block.hash,
        vec![coinbase, invalid_transaction],
    );

    // it should reject the block
    let err = validate_block(&db, &block).unwrap_err();
    let inner_err = err.downcast::<TransactionError>().unwrap();
    assert!(matches!(
        inner_err,
        TransactionError::SenderAccountDoesNotExist
    ));
}
