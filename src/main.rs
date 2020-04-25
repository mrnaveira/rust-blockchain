mod api;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    println!("{:?}", blockchain.current_block);

    blockchain.add_block(Vec::new());
    println!("{:?}", blockchain.current_block);

    blockchain.add_block(Vec::new());
    println!("{:?}", blockchain.current_block);

    let port = 8000;
    api::run(port).expect("could not start the API");
}