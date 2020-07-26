# rust-blockchain
A toy blockchain written in Rust. This blockchain is a conceptual work:
* Defines data structures to model a minimum blockchain
* Mines new blocks in a separate thread, running PoW with a fixed difficulty. Mining will stop until new transactions arrive.
* Provides a REST API to retrieve the blocks and add transactions

## Getting Started
You will need Rust installed. You can install it from https://rustup.rs. It will also install Cargo, Rust's package manager.

Then, simple compile and run the project:
```console
$ cargo build
$ ./target/debug/rust_blockchain
```

Which will create a blockchain and start listening on HTTP port 8000 for incoming REST requests.

## REST API
| Method | Resource | Description | Example
| --- | --- | --- | --- |
| GET | /blocks | List all blocks of the blockchain | `curl -X GET http://localhost:8000/blocks`
| POST | /transactions | Add a new transaction to the pool | `curl -X POST http://localhost:8000/transactions -H 'Content-Type: application/json' -d '{"sender": "1", "recipient": "2", "amount": 1002}'`

## Roadmap

* ~~Hello world REST API in Rust~~
* ~~Structs to represent blockchain data~~
* ~~API methods to add transactions and check blocks~~
* ~~Transaction pool that holds not realized transactions~~
* ~~Basic miner that adds transactions every N seconds~~
* ~~Basic PoW implementation: nonce, miner calculates hashes and fixed difficulty~~
* API methods to start, stop and overview mining
* Mining peers communicate new blocks over WebSockets
* PoW improvement: dynamic difficulty
* Transaction validation: check for correct balances and digital signing
* Block rewards (subsidy + transaction fees) and halving
* Other: disk storage of blocks, logging, config file, CLI, testing, etc.