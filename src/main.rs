mod blockchain;

use blockchain::{
    Blockchain,
};

fn main() {
    let mut blockchain = Blockchain::new();
    println!("Current block {:?} - {:?}", blockchain.current_block.index, blockchain.current_block.timestamp);

    blockchain.add_block(Vec::new());
    println!("Current block {:?} - {:?}", blockchain.current_block.index, blockchain.current_block.timestamp);
}