use crate::types::{Address, Block, Coin, Network};

pub trait Database {
    fn get_network(&self) -> Network;
    fn get_all_blocks(&self) -> Vec<Block>;
    fn get_tip_block(&self) -> Option<Block>;
    fn get_account_balance(&self, address: &Address) -> Option<Coin>;
}
