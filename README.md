# rust-blockchain
A simple blockchain example written in Rust:
* Defines data structures to model a minimum blockchain
* Mines new blocks in a separate thread, running PoW with a fixed difficulty. Mining will stop until new transactions arrive.
* Provides a REST API to retrieve the blocks and add transactions

## Getting Started
You will need Rust and Cargo installed.

Then, simply compile and run the executable:
```console
$ cargo build --release
$ ./target/release/rust_blockchain
```

Which will create a blockchain and start listening on HTTP port `8000` for incoming REST requests.

## REST API
| Method | URL | Description
| --- | --- | --- |
| GET | /blocks | List all blocks of the blockchain <br /> `curl -X GET http://localhost:8000/blocks`
| POST | /transactions | Add a new transaction to the pool <br /> `curl -X POST http://localhost:8000/transactions -H 'Content-Type: application/json' -d '{"sender": "1", "recipient": "2", "amount": 1002}'`

The file `rest_api.postman_collection.json` contains a Postman collection with examples of all requests.

## Roadmap

- [x] Boilerplate REST API in Rust
- [x] Structs to represent blockchain data
- [x] API methods to add transactions and check blocks
- [x] Transaction pool that holds not realized transactions
- [x] Basic miner that adds transactions every N seconds
- [x] Basic PoW implementation: nonce, miner calculates hashes and fixed difficulty
- [ ] API methods to start, stop and overview mining
- [ ] Mining peers communicate new blocks over WebSockets
- [ ] PoW improvement: dynamic difficulty
- [ ] Transaction validation: check for correct balances and digital signing
- [ ] Block rewards (subsidy + transaction fees) and halving
- [ ] Other: disk storage of blocks, logging, config file, CLI, testing, etc.