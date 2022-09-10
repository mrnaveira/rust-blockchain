use crate::types::{Address, Block, Coin, Network};

pub trait Database {
    // network related methods
    fn get_network(&self) -> Network;

    // block related methods
    fn get_all_blocks(&self) -> Vec<Block>;
    fn get_tip_block(&self) -> Option<Block>;
    //fn append_block(&mut self, block: &Block);

    // transaction related methods
    // fn get_mempool(&self) -> Vec<Transaction>;
    //fn add_to_mempool(&mut self, transaction: &Transaction);
    //fn remove_from_mempool(&mut self, transactions: &[Transaction]);

    // account related methods
    fn get_account_balance(&self, address: &Address) -> Option<Coin>;
    // fn add_funds(&mut self, address: &Address, amount: Coin);
    // fn transfer(&mut self, sender: &Address, recipient: &Address, amount: Coin) -> Result<()>;
}
