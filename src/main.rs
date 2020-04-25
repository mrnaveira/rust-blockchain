mod blockchain;

use blockchain::{
    Blockchain
};

fn main() {
    let mut blockchain = Blockchain::new();
    println!("{:?}", blockchain.current_block);

    blockchain.add_block(Vec::new());
    println!("{:?}", blockchain.current_block);

    blockchain.add_block(Vec::new());
    println!("{:?}", blockchain.current_block);
}