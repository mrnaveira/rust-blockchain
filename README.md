# rust-blockchain

![example workflow](https://github.com/mrnaveira/rust-blockchain/actions/workflows/build.yaml/badge.svg) ![example workflow](https://github.com/mrnaveira/rust-blockchain/actions/workflows/lint.yaml/badge.svg) ![example workflow](https://github.com/mrnaveira/rust-blockchain/actions/workflows/test.yaml/badge.svg) [![Coverage Status](https://coveralls.io/repos/github/mrnaveira/rust-blockchain/badge.svg?service=github)](https://coveralls.io/github/mrnaveira/rust-blockchain)

A Proof of Work blockchain written in Rust. For educational purposes only.

Features:
* Defines data structures to model a minimum blockchain
* Mines new blocks in a separate thread, running a Proof of Work algorithm with a fixed difficulty
* Synchronizes new blocks with peer nodes in a decentralized network
* Provides a REST API to retrieve the blocks and add transactions

## Getting Started
You will need Rust and Cargo installed.

```bash
# Download the code
$ git clone https://github.com/mrnaveira/rust-blockchain
$ cd rust-blockchain

# Run all tests
$ cargo test

# Build the project in release mode
$ cargo build --release

# Run the application
$ ./target/release/rust_blockchain
```

The application will start mining and listening on port `8000` for incoming client requests via a REST API. To change any environment variable (port, mining parameters, etc.) refer to the `.env.example` file.

For development setup, check the [development notes section](#development-notes).

## Client REST API
The application provides a REST API for clients to operate with the blockchain.

| Method | URL | Description
| --- | --- | --- |
| GET | /blocks | List all blocks of the blockchain
| POST | /blocks | Append a new block to the blockchain
| POST | /transactions | Add a new transaction to the pool

The file `doc/rest_api.postman_collection.json` contains a Postman collection with examples of all requests.

## Block Structure

In a blockchain, transactions are grouped into blocks. Aside from transactions, a block contains metadata needed to secure and maintain the sequence in the chain. This sequence of blocks is key to allow transactions to occur in order.

The `model` module in this project contains the data structures to model the blockchain, as described in the next diagram:

![Blockchain structure diagram](./doc/blockchain_structure.png)

Each block contains the following data:
* **index**: position of the block in the blockchain
* **timestamp**: date and time of block creation
* **nonce**: arbitrary number that makes the block, when hashed, meet the mining difficulty restriction. Is the number that miners are competing to get first
* **previous_hash**: hash of the previous block in the chain. Allows to maintain order of blocks in the blockchain. There is an exception with the first block of the chain (genesis block) which has no previous_hash
* **hash**: hash of the block including all fields
* **transactions**: a list of all transactions included in the block. Each transaction has a **sender**, **recipient** and **amount**.

## Proof of Work

Proof of Work (PoW) is a common consensus algorithm used widely in most cryptocurrencies like Bitcoin. A participant node in the network that wants to add new transactions in the blockchain (and get the rewards for it) must prove that a certain amount of computational work has been done. This work can take a large amount of time to do but at the same time it's very easy to validate by other nodes.

This prevents the double spending problem by forcing any attacker that wants to remove or modify a transaction to redo all the computational work from the target block to the current one. The attacker must have a larger computational capacity than the rest of the network combined to be able to achieve it (51% attack). 

This project implements a simplified PoW algorithm based on hashes, in the line of what Bitcoin does. The `miner.rs` file implements the steps to create a valid block:
1. All transactions in the pool are added to the block. If there is no transactions in the pool, do not mine until they arrive.
2. The block contains the valid index and timestamp, as well as the **hash of the previous block** to maintain order.
3. Iterate the **nonce** value until the hash of the whole block satisfies the difficulty constraint, which is to be less than a target value. The difficulty target is fixed for the execution of the server, but in a real project we would want dynamic difficulty adjusted in runtime to have constant time intervals between blocks.
4. When a valid block is found, add it to the blockchain and repeat from step 1 to create the next block.

## Development notes

### Git hooks
This project uses [cargo-husky](https://github.com/rhysd/cargo-husky) to setup a Git pre-commit hook to check code style (using [clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt)), cargo dependencies and run all tests. If any of those tasks fails, the hook prevents you to commit the changes.

To automatically create the hooks in your local git repository, simply run all tests the first time:
```bash
$ cargo test
```

### GitHub Actions
There are also multiple GitHub Actions (using [actions-rs](https://github.com/actions-rs)) under the `.github/workflows` folder, as a form of CI. On each commit or PR they perform similar checks as the Git hooks (clippy/rustfmt, dependencies/build and test) plus the test coverage (explained in the [coverage section](#test-coverage) ). The results are displayed as badges below the title of this README.

### Test organization
The test organization follows the [recommended guidelines for Rust](https://doc.rust-lang.org/book/ch11-03-test-organization.html):
* **Unit tests** are located inside the file with the code they're testing, inside a module annotated with `cfg(test)`.
* **Integration tests** are located inside the `tests` folder. This project is a server application and not a library, so the integration tests run the server in a child OS thread, perform real REST API calls and then terminate the process. This way we test all parts of the application using only the REST API, treating it as a black box.

### Test coverage
To generate the test coverage report, at the moment it's required to use the nightly version of Rust. Also you need to install `grconv` and `llvm-tools`.
The detailed instructions are [in the grcov repository](https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project) as well as in the `scripts/coverage_report.sh` script.

Then, each time we want to to generate the coverage report, we simply execute the script:
```bash
$ ./scripts/coverage_report.sh
```

The results will be availabe under the `coverage` folder for inspection. Also, there is a GitHub Action (in `.github/workflows/coverage.yaml`) that will automatically calculate it on every push to `origin` and display the coverage in a badge under the title of this README. 

### Concurrency implementation

In this project, the `main` thread spawns three OS threads:
* One for the **miner**. As mining is very computationally-intensive, we want a dedicated OS thread to not slow down other operations in the application. In a real blockchain we would also want parallel mining (by handling a different subrange of nonces in each thread), but for simplicity we will only use one thread.
* Other thread for the **REST API**. The API uses [`actix-web`](https://github.com/actix/actix-web), which internally uses [`tokio`](https://crates.io/crates/tokio), so it's optimized for asynchronous operations.
* A thread for the **peer system**, that periodically sends and receives new blocks from peers over the network.

Thread spawning and handling is implemented using [`crossbeam-utils`](https://crates.io/crates/crossbeam-utils) to reduce boilerplate code from the standard library.

Also, all threads share data, specifically the **block list** and the **transaction pool**. Those two data structures are implemented by using `Arc<Mutex>` to allow multiple concurrent writes and reads in a safe way from separate threads.

## Roadmap

- [x] Boilerplate REST API in Rust
- [x] Structs to represent blockchain data
- [x] API methods to add transactions and check blocks
- [x] Transaction pool that holds not realized transactions
- [x] Basic miner that adds transactions every N seconds
- [x] Basic PoW implementation: nonce, miner calculates hashes and fixed difficulty
- [x] Mining peers communicate new blocks over the network
- [ ] Block rewards: subsidy and transaction fees
- [ ] Validate transaction balances
- [ ] Dynamic difficulty (aiming for constant time intervals between blocks)
- [ ] Halving
- [ ] Blockchain disk storage
- [ ] Digital signing of transactions