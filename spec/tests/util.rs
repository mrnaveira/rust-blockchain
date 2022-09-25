use anyhow::Result;
use spec::{
    types::{hash::ConsensusHashable, Address, Block, Coin, Network, Transaction},
    validators::{validate_block_transactions, BLOCK_SUBSIDY},
    Database,
};
use std::collections::HashMap;

pub struct MockDatabase {
    network: Network,
    blocks: Vec<Block>,
    balances: HashMap<Address, Coin>,
    transactions: Vec<Transaction>,
}

impl MockDatabase {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            blocks: vec![],
            balances: HashMap::new(),
            transactions: vec![],
        }
    }

    pub fn append_genesis_block(&mut self) -> Result<()> {
        let coinbase = build_coinbase_transaction();
        let block = Block::new(0, 0, self.network.consensus_hash(), vec![coinbase]);
        self.append_block(&block)
    }

    pub fn append_block(&mut self, block: &Block) -> Result<()> {
        self.process_transactions(block)?;
        self.blocks.push(block.clone());

        Ok(())
    }

    pub fn process_transactions(&mut self, block: &Block) -> Result<()> {
        // make sure that the transactions in the block are valid
        validate_block_transactions(self, block)?;

        let mut transactions = block.transactions.iter();

        // after validation, we know that the coinbase exists and it's valid
        let coinbase = transactions.next().unwrap();
        self.add_funds(&coinbase.recipient, coinbase.amount);

        // same as before, at this point we know that all the regular transactions are valid
        for transaction in transactions {
            self.transfer(
                &transaction.sender,
                &transaction.recipient,
                transaction.amount,
            );
        }

        Ok(())
    }

    fn add_funds(&mut self, address: &Address, amount: Coin) {
        *self.balances.entry(address.clone()).or_insert(0) += amount;
    }

    fn substract_funds(&mut self, address: &Address, amount: Coin) {
        *self.balances.entry(address.clone()).or_insert(0) -= amount;
    }

    fn transfer(&mut self, sender: &Address, recipient: &Address, amount: Coin) {
        self.substract_funds(sender, amount);
        self.add_funds(recipient, amount);
    }
}

impl Default for MockDatabase {
    fn default() -> Self {
        let network = Network {
            description: "Test network".to_string(),
            difficulty: 0,
            timestamp: 0,
        };

        MockDatabase::new(network.clone())
    }
}

impl Database for MockDatabase {
    fn get_network(&self) -> Network {
        self.network.clone()
    }

    fn get_all_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    fn get_tip_block(&self) -> Option<Block> {
        match self.blocks.last() {
            Some(block) => Some(block.clone()),
            None => None,
        }
    }

    fn get_account_balance(&self, address: &Address) -> Option<Coin> {
        match self.balances.get(address) {
            Some(balance) => Some(*balance),
            None => None,
        }
    }

    fn get_mempool_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }
}

pub fn build_coinbase_transaction() -> Transaction {
    Transaction {
        sender: Address::default(),
        recipient: miner_address(),
        amount: BLOCK_SUBSIDY,
    }
}

pub fn miner_address() -> Address {
    Address::try_from(
        "fe8aa8cc6011898d49bdacd0ab52075e92e1dfb2915bb9223528a5737583731d".to_string(),
    )
    .unwrap()
}

pub fn alice() -> Address {
    Address::try_from(
        "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string(),
    )
    .unwrap()
}
