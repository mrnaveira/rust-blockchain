# rust-blockchain
A toy blockchain written in Rust. Blockchain is a difficult subject to understand only 
by reading about it. Decided to learn by doing, this repository attempts to provide
a simple implementation of a blockhain. 

The language of choice is Rust due to it being memory safe, fast and great at concurrent programming.

This repository is only a conceptual work:
* Defines data structures to model a minimum blockchain
* Provides a REST API to retrieve the blocks and add transactions
* Every 3 transactions, a new block will be added to the blockchain

## Getting Started
You will need Rust installed. You can install it from https://rustup.rs. It will also install Cargo, Rust's package manager.

Then, simple compile and run the project:
```console
$ cargo build
$ ./target/debug/rust_blockchain
```

Which will create a blockchain and start listening on HTTP port 8000 for incoming REST requests.

## API
| Method | Resource | Description | Example
| --- | --- | --- | --- |
| GET | /blocks | List all blocks of the blockchain | `curl -X GET http://localhost:8000/blocks`
| POST | /transactions | Add a new transaction to the pool | `curl -X POST http://localhost:8000/transactions -H 'Content-Type: application/json' -d '{"sender": "1", "recipient": "2", "amount": 1002}'`

