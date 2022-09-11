mod accounts;
mod blocks;
mod mempool;

use std::sync::{Arc, RwLock};

use anyhow::Result;
use spec::types::{Address, Block, Coin, Network, Transaction};
use spec::validators::{validate_block, validate_transaction};
use spec::Database as SpecDatabase;

use self::accounts::AccountDatabase;
use self::blocks::BlockDatabase;
use self::mempool::Mempool;

struct State {
    network: Network,
    block_db: BlockDatabase,
    account_db: AccountDatabase,
    mempool: Mempool,
}

// the caller does not need to worry about concurrency when calling methods
// as concurrent atomic writes from different threads are supported
#[derive(Clone)]
pub struct Database {
    state: Arc<RwLock<State>>,
}

impl Database {
    pub fn new(network: Network) -> Self {
        let state = State {
            network,
            block_db: BlockDatabase::default(),
            account_db: AccountDatabase::default(),
            mempool: Mempool::default(),
        };

        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    pub fn append_block(&self, block: &Block) -> Result<()> {
        // get a write lock on the data, so the whole operation is atomic
        let mut state = self.state.write().unwrap();

        // make sure the block is valid before any other operation
        validate_block(self, block)?;

        // append the new block to the end of the chain
        state.block_db.append_block(block.clone());

        // update account balances
        Self::process_transactions(&mut state.account_db, block)?;

        // submitted transactions should be removed from the mempool
        state.mempool.remove_transactions(&block.transactions);

        Ok(())
    }

    fn process_transactions(account_db: &mut AccountDatabase, block: &Block) -> Result<()> {
        // we know that at this point the block was already fully validated
        // so we just update the balances without further validation
        let mut transactions = block.transactions.iter();

        // process the coinbase transaction, rewarding the miner
        let coinbase = transactions.next().unwrap();
        account_db.add_funds(&coinbase.recipient, coinbase.amount);

        // process transfers between accounts
        for transaction in transactions {
            account_db.transfer(
                &transaction.sender,
                &transaction.recipient,
                transaction.amount,
            )?;
        }

        Ok(())
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        let state = self.state.read().unwrap();
        state.mempool.get_transactions()
    }

    pub fn add_transaction(&self, transaction: Transaction) -> Result<()> {
        let mut state = self.state.write().unwrap();
        validate_transaction(self, &transaction)?;
        state.mempool.add_transaction(transaction);

        Ok(())
    }
}

// this trait is necessary for the spec validators to run
impl SpecDatabase for Database {
    fn get_network(&self) -> Network {
        let state = self.state.read().unwrap();
        state.network.clone()
    }

    fn get_all_blocks(&self) -> Vec<Block> {
        let state = self.state.read().unwrap();
        state.block_db.get_all_blocks()
    }

    fn get_tip_block(&self) -> Option<Block> {
        let state = self.state.read().unwrap();
        state.block_db.get_tip_block()
    }

    fn get_account_balance(&self, address: &Address) -> Option<Coin> {
        let state = self.state.read().unwrap();
        state.account_db.get_account_balance(address)
    }
}
