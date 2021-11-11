use ethereum_types::U256;
use isahc::{Body, ReadResponseExt, Request, Response};
use serde::{Deserialize, Serialize};

use super::server::Server;

pub type BlockHash = U256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

pub trait Api {
    fn get_blocks(&self) -> Vec<Block>;
    fn get_last_block(&self) -> Block;
    fn add_block(&self, block: &Block) -> Response<Body>;
    fn add_valid_block(&self) -> Response<Body>;
    fn add_transaction(&self, transaction: &Transaction) -> Response<Body>;
}

impl Api for Server {
    fn get_blocks(&self) -> Vec<Block> {
        // list the blocks by querying the REST API
        let uri = format!("{}/blocks", get_base_url(self));
        let mut response = isahc::get(uri).unwrap();

        // check that the response is sucessful
        assert_eq!(response.status().as_u16(), 200);

        // parse the list of blocks from the response body
        let raw_body = response.text().unwrap();
        let blocks: Vec<Block> = serde_json::from_str(&raw_body).unwrap();

        blocks
    }

    fn get_last_block(&self) -> Block {
        self.get_blocks().last().unwrap().to_owned()
    }

    fn add_valid_block(&self) -> Response<Body> {
        let last_block = self.get_last_block();
        let valid_block = Block {
            index: last_block.index + 1,
            timestamp: 0,
            nonce: 0,
            // the previous hash is checked
            previous_hash: last_block.hash,
            // the api automatically recalculates the hash...
            // ...so no need to add a valid one here
            hash: BlockHash::default(),
            transactions: [].to_vec(),
        };
        self.add_block(&valid_block)
    }

    fn add_block(&self, block: &Block) -> Response<Body> {
        // send the request to the REST API
        let uri = format!("{}/blocks", get_base_url(self));
        let body = serde_json::to_string(&block).unwrap();

        post_request(uri, body)
    }

    fn add_transaction(&self, transaction: &Transaction) -> Response<Body> {
        // send the request to the REST API
        let uri = format!("{}/transactions", get_base_url(self));
        let body = serde_json::to_string(&transaction).unwrap();

        post_request(uri, body)
    }
}

fn get_base_url(server: &Server) -> String {
    format!("http://localhost:{}", server.config.port)
}

fn post_request(uri: String, body: String) -> Response<Body> {
    let request = Request::post(uri)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    isahc::send(request).unwrap()
}
