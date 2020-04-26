mod api;
mod blockchain;

use blockchain::{Blockchain, Transaction};

fn main() {
    // genesis block
    let mut blockchain = Blockchain::new();

    // block 1
    let mut transactions_block_1 = Vec::new();
    transactions_block_1.push(Transaction {
        sender: String::from("1"),
        recipient: String::from("2"),
        amount: 1000
    });
    blockchain.add_block(transactions_block_1);

    // block 2
    let mut transactions_block_2 = Vec::new();
    transactions_block_2.push(Transaction {
        sender: String::from("2"),
        recipient: String::from("1"),
        amount: 500
    });
    transactions_block_2.push(Transaction {
        sender: String::from("2"),
        recipient: String::from("3"),
        amount: 500
    });
    blockchain.add_block(transactions_block_2);

    let port = 8000;
    api::run(port, blockchain).expect("could not start the API");
}