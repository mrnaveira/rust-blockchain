use assert_cmd::cargo::cargo_bin;
use isahc::prelude::*;
use std::{convert::TryInto, process::Command, time::Duration};

use std::thread;

use nix::{
    sys::signal::{kill, Signal::SIGTERM},
    unistd::Pid,
};

mod common;

#[test]
#[cfg(unix)]
fn test_new_blockchain() {
    // start the blockchain in the background
    let mut cmd = Command::new(cargo_bin("rust_blockchain")).spawn().unwrap();

    // wait for the blockchain to start
    thread::sleep(Duration::from_secs(1));
    assert!(
        cmd.try_wait().unwrap().is_none(),
        "the process should still be running"
    );

    // list the blocks by querying the REST API
    let mut response = isahc::get("http://localhost:8000/blocks").unwrap();

    // check that the response is sucessful
    assert_eq!(response.status().as_u16(), 200);

    // parse the list of blocks from the response body
    let raw_body = response.text().unwrap();
    let blocks: Vec<common::Block> = serde_json::from_str(&raw_body).unwrap();

    // check that the blocks only contain the genesis block
    assert_eq!(blocks.len(), 1);
    let block = blocks.first().unwrap();

    // check that the genesis block fields are valid
    assert_eq!(block.index, 0);
    assert_eq!(block.nonce, 0);
    assert_eq!(block.previous_hash, common::BlockHash::default());
    assert!(block.transactions.is_empty());

    // finish the blockchain instance so it does not become a zombie process
    kill(Pid::from_raw(cmd.id().try_into().unwrap()), SIGTERM).unwrap();
}
