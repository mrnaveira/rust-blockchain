use ethereum_types::U256;
use isahc::{Body, ReadResponseExt, Request, Response};
use serde::{Deserialize, Serialize};

use crate::common::server_utils::get_server_url;

pub type BlockHash = U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

pub fn get_blocks() -> Vec<Block> {
    // list the blocks by querying the REST API
    let uri = format!("{}/blocks", get_server_url());
    let mut response = isahc::get(uri).unwrap();

    // check that the response is sucessful
    assert_eq!(response.status().as_u16(), 200);

    // parse the list of blocks from the response body
    let raw_body = response.text().unwrap();
    let blocks: Vec<Block> = serde_json::from_str(&raw_body).unwrap();

    blocks
}

pub fn add_block(block: &Block) -> Response<Body> {
    // send the request to the REST API
    let uri = format!("{}/blocks", get_server_url());
    let body = serde_json::to_string(&block).unwrap();

    post_request(uri, body)
}

pub fn add_transaction(transaction: &Transaction) -> Response<Body> {
    // send the request to the REST API
    let uri = format!("{}/transactions", get_server_url());
    let body = serde_json::to_string(&transaction).unwrap();

    post_request(uri, body)
}

fn post_request(uri: String, body: String) -> Response<Body> {
    let request = Request::post(uri)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    isahc::send(request).unwrap()
}
