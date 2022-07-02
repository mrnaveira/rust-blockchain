use anyhow::Result;
use std::{
    slice::Iter,
    sync::{Arc, Mutex},
};
use thiserror::Error;

use super::{account_balance_map::AccountBalanceMap, Block, BlockHash, Transaction};

pub type BlockVec = Vec<Block>;

// We don't need to export this because concurrency is encapsulated in this file
type SyncedBlockVec = Arc<Mutex<BlockVec>>;
type SyncedAccountBalanceVec = Arc<Mutex<AccountBalanceMap>>;

pub const BLOCK_SUBSIDY: u64 = 100;

// Error types to return when trying to add blocks with invalid fields
#[derive(Error, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BlockchainError {
    #[error("Invalid index")]
    InvalidIndex,

    #[error("Invalid previous_hash")]
    InvalidPreviousHash,

    #[error("Invalid hash")]
    InvalidHash,

    #[error("Invalid difficulty")]
    InvalidDifficulty,

    #[error("Coinbase transaction not found")]
    CoinbaseTransactionNotFound,

    #[error("Invalid coinbase amount")]
    InvalidCoinbaseAmount,
}

// Struct that holds all the blocks in the blockhain
// Multiple threads can read/write concurrently to the list of blocks
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub difficulty: u32,
    blocks: SyncedBlockVec,
    account_balances: SyncedAccountBalanceVec,
}

// Basic operations in the blockchain are encapsulated in the implementation
// Encapsulates concurrency concerns, so external callers do not need to know how it's handled
impl Blockchain {
    // Creates a brand new blockchain with a genesis block
    pub fn new(difficulty: u32) -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        // add the genesis block to the synced vec of blocks
        let blocks = vec![genesis_block];
        let synced_blocks = Arc::new(Mutex::new(blocks));
        let synced_account_balances = SyncedAccountBalanceVec::default();

        Blockchain {
            difficulty,
            blocks: synced_blocks,
            account_balances: synced_account_balances,
        }
    }

    fn create_genesis_block() -> Block {
        let index = 0;
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        let mut block = Block::new(index, nonce, previous_hash, transactions);

        // to easily sync multiple nodes in a network, the genesis blocks must match
        // so we clear the timestamp so the hash of the genesis block is predictable
        block.timestamp = 0;
        block.hash = block.calculate_hash();

        block
    }

    // Returns a copy of the most recent block in the blockchain
    pub fn get_last_block(&self) -> Block {
        let blocks = self.blocks.lock().unwrap();

        blocks[blocks.len() - 1].clone()
    }

    // Returns a copy of the whole list of blocks
    pub fn get_all_blocks(&self) -> BlockVec {
        let blocks = self.blocks.lock().unwrap();

        blocks.clone()
    }

    // Tries to append a new block into the blockchain
    // It will validate that the values of the new block are consistend with the blockchain state
    // This operation is safe to be called concurrently from multiple threads
    pub fn add_block(&self, block: Block) -> Result<()> {
        // the "blocks" attribute is protected by a Mutex
        // so only one thread at a time can access the value when the lock is held
        // that prevents adding multiple valid blocks at the same time
        // preserving the correct order of indexes and hashes of the blockchain
        let mut blocks = self.blocks.lock().unwrap();
        let last = &blocks[blocks.len() - 1];

        // check that the index is valid
        if block.index != last.index + 1 {
            return Err(BlockchainError::InvalidIndex.into());
        }

        // check that the previous_hash is valid
        if block.previous_hash != last.hash {
            return Err(BlockchainError::InvalidPreviousHash.into());
        }

        // check that the hash matches the data
        if block.hash != block.calculate_hash() {
            return Err(BlockchainError::InvalidHash.into());
        }

        // check that the difficulty is correct
        if block.hash.leading_zeros() < self.difficulty {
            return Err(BlockchainError::InvalidDifficulty.into());
        }

        // update the account balances by processing the block transactions
        self.update_account_balances(&block.transactions)?;

        // append the block to the end
        blocks.push(block);

        Ok(())
    }

    fn update_account_balances(&self, transactions: &[Transaction]) -> Result<()> {
        let mut account_balances = self.account_balances.lock().unwrap();
        // note that if any transaction (including coinbase) is invalid, an error will be returned before updating the balances
        let new_account_balances =
            Blockchain::calculate_new_account_balances(&account_balances, transactions)?;
        *account_balances = new_account_balances;

        Ok(())
    }

    fn calculate_new_account_balances(
        account_balances: &AccountBalanceMap,
        transactions: &[Transaction],
    ) -> Result<AccountBalanceMap> {
        // we work on a copy of the account balances
        let mut new_account_balances = account_balances.clone();
        let mut iter = transactions.iter();

        // the first transaction is always the coinbase transaction
        // in which the miner receives the mining rewards
        Blockchain::process_coinbase(&mut new_account_balances, iter.next())?;

        // the rest of the transactions are regular transfers between accounts
        Blockchain::process_transfers(&mut new_account_balances, iter)?;

        Ok(new_account_balances)
    }

    fn process_coinbase(
        account_balances: &mut AccountBalanceMap,
        coinbase: Option<&Transaction>,
    ) -> Result<()> {
        // The coinbase transaction is required in a valid block
        let coinbase = match coinbase {
            Some(transaction) => transaction,
            None => return Err(BlockchainError::CoinbaseTransactionNotFound.into()),
        };

        // In coinbase transactions, we only need to check that the amount is valid,
        // because whoever provides a valid proof-of-work block can receive the new coins
        let is_valid_amount = coinbase.amount == BLOCK_SUBSIDY;
        if !is_valid_amount {
            return Err(BlockchainError::InvalidCoinbaseAmount.into());
        }

        // The amount is valid so we add the new coins to the miner's address
        account_balances.add_amount(&coinbase.recipient, coinbase.amount);

        Ok(())
    }

    fn process_transfers(
        new_account_balances: &mut AccountBalanceMap,
        transaction_iter: Iter<Transaction>,
    ) -> Result<()> {
        // each transaction is validated using the updated account balances from previous transactions
        // that means that we allow multiple transacions from the same address in the same block
        // as long as they are consistent
        for tx in transaction_iter {
            new_account_balances.transfer(&tx.sender, &tx.recipient, tx.amount)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        account_balance_map::AccountBalanceMapError,
        test_util::{alice, bob, carol},
        Address, Transaction,
    };

    use super::*;

    const NO_DIFFICULTY: u32 = 0;

    #[test]
    fn should_have_valid_genesis_block() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // check that a new blockchain has one and only one block
        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 1);

        // check that the last block is in the blockchain
        let block = blockchain.get_last_block();
        assert_eq!(block.hash, blocks[0].hash);

        // check that the genesis block has valid values
        assert_eq!(block.index, 0);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.previous_hash, BlockHash::default());
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn should_let_adding_valid_blocks() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a valid block
        let previous_hash = blockchain.get_last_block().hash;
        let coinbase = Transaction {
            sender: Address::default(), // sender is ignored in coinbases
            recipient: bob(),
            amount: BLOCK_SUBSIDY,
        };
        let tx1 = Transaction {
            sender: bob(),
            recipient: alice(),
            amount: 5,
        };
        let tx2 = Transaction {
            sender: alice(),
            recipient: bob(),
            amount: 5,
        };
        let block = Block::new(1, 0, previous_hash, vec![coinbase, tx1, tx2]);

        // add it to the blockchain and check it was really added
        let result = blockchain.add_block(block.clone());
        assert!(result.is_ok());

        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 2);

        let last_block = blockchain.get_last_block();
        assert_eq!(last_block.hash, block.hash);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_index() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with invalid index
        let invalid_index = 2;
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(invalid_index, 0, previous_hash, Vec::new());

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidIndex);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_previous_hash() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with invalid previous hash
        let invalid_previous_hash = BlockHash::default();
        let block = Block::new(1, 0, invalid_previous_hash, Vec::new());

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidPreviousHash);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_hash() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with invalid hash
        let previous_hash = blockchain.get_last_block().hash;
        let mut block = Block::new(1, 0, previous_hash, Vec::new());
        block.hash = BlockHash::default();

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidHash);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_difficulty() {
        // set up a blockchain with an insane difficulty
        let difficulty: u32 = 30;
        let blockchain = Blockchain::new(difficulty);

        // create a valid block
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(1, 0, previous_hash, Vec::new());

        // ensure that the hash actually does NOT meet the difficulty
        assert!(block.hash.leading_zeros() < difficulty);

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidDifficulty);
    }

    #[test]
    fn should_not_let_adding_block_with_no_coinbase() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block without a coinbase
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(1, 0, previous_hash, vec![]);

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::CoinbaseTransactionNotFound);
    }

    #[test]
    fn should_not_let_adding_block_with_invalid_coinbase() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a block with an invalid coinbase amount
        let previous_hash = blockchain.get_last_block().hash;
        let coinbase = Transaction {
            sender: Address::default(),
            recipient: Address::default(),
            amount: BLOCK_SUBSIDY + 1,
        };
        let block = Block::new(1, 0, previous_hash, vec![coinbase]);

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_err(result, BlockchainError::InvalidCoinbaseAmount);
    }

    #[test]
    fn should_not_let_add_transaction_with_insufficient_funds() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create an invalid block
        let previous_hash = blockchain.get_last_block().hash;
        // the coinbase is valid
        let coinbase = Transaction {
            sender: Address::default(), // sender is ignored in coinbases
            recipient: bob(),
            amount: BLOCK_SUBSIDY,
        };
        // but the following transaction has an invalid amount
        let invalid_transaction = Transaction {
            sender: bob(),
            recipient: alice(),
            // the amount is greated than what bob has
            amount: BLOCK_SUBSIDY + 1,
        };
        let block = Block::new(1, 0, previous_hash, vec![coinbase, invalid_transaction]);

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_balance_err(result, AccountBalanceMapError::InsufficientFunds);
    }

    #[test]
    fn should_not_let_add_transaction_with_non_existent_sender() {
        let blockchain = Blockchain::new(NO_DIFFICULTY);

        // create a valid block
        let previous_hash = blockchain.get_last_block().hash;
        // the coinbase is valid
        let coinbase = Transaction {
            sender: Address::default(), // sender is ignored in coinbases
            recipient: bob(),
            amount: BLOCK_SUBSIDY,
        };
        // but the sender does not exist
        let invalid_transaction = Transaction {
            // the sender address do not have any funds from previous transactions
            sender: carol(),
            recipient: bob(),
            amount: 1,
        };
        let block = Block::new(1, 0, previous_hash, vec![coinbase, invalid_transaction]);

        // try adding the invalid block, it should return an error
        let result = blockchain.add_block(block.clone());
        assert_balance_err(result, AccountBalanceMapError::SenderAccountDoesNotExist);
    }

    fn assert_err(result: Result<(), anyhow::Error>, error_type: BlockchainError) {
        let err = result.unwrap_err().downcast::<BlockchainError>().unwrap();
        assert_eq!(err, error_type);
    }

    fn assert_balance_err(result: Result<(), anyhow::Error>, error_type: AccountBalanceMapError) {
        let err = result
            .unwrap_err()
            .downcast::<AccountBalanceMapError>()
            .unwrap();
        assert_eq!(err, error_type);
    }
}
