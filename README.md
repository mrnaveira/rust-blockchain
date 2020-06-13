# rust-blockchain
A toy blockchain written in Rust.
Blockchain is a difficult subject to understand only by reading about it. Decided to learn by doing, this repository attempts to provide a simple implementation of a blockchain.
The language of choice is Rust due to it being memory safe, fast and great at concurrent programming.
The consensus mechanism will be proof-of-work (PoW)
as it's the most used one in public blockchains.

This repository is a conceptual work. For now, it only:
* Defines data structures to model a minimum blockchain
* Provides a REST API to retrieve the blocks and add transactions
* Every 5 seconds a new block will be added to the blockchain, containing all transactions in the transaction pool. In the near future, a PoW system will be implemented.

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
* Basic PoW implementation: nonce, miner calculates hashes and fixed difficulty
* API methods to start, stop and overview mining
* Mining peers communicate new blocks over WebSockets
* PoW improvement: dynamic difficulty
* Transaction validation: check for correct balances and digital signing
* Block rewards (subsidy and transactions fees) and halving
* Other: disk storage of blocks, logging, config file, CLI, testing, etc.