use isahc::{ReadResponseExt, Request};
use spec::{Block, Transaction};

pub trait NodeClient {
    fn get_last_block(&self) -> Block;
    fn get_mempool_transactions(&self) -> Vec<Transaction>;
    fn submit_block(&self, block: &Block);
}

pub struct NetworkNodeClient {
    pub node_url: String,
}

impl NetworkNodeClient {
    pub fn new(node_url: String) -> Self {
        NetworkNodeClient { node_url }
    }
}

impl NodeClient for NetworkNodeClient {
    fn get_last_block(&self) -> Block {
        let uri = format!("{}/blocks", self.node_url);
        let mut response = isahc::get(uri).unwrap();

        // check that the response is sucessful
        assert_eq!(response.status().as_u16(), 200);

        // parse and return the list of blocks from the response body
        let raw_body = response.text().unwrap();
        let blocks: Vec<Block> = serde_json::from_str(&raw_body).unwrap();

        blocks.last().unwrap().to_owned()
    }

    fn get_mempool_transactions(&self) -> Vec<Transaction> {
        let uri = format!("{}/transactions", self.node_url);
        let mut response = isahc::get(uri).unwrap();

        // check that the response is sucessful
        assert_eq!(response.status().as_u16(), 200);

        // parse and return the list of transactions from the response body
        let raw_body = response.text().unwrap();
        serde_json::from_str(&raw_body).unwrap()
    }

    fn submit_block(&self, block: &Block) {
        let uri = format!("{}/blocks", self.node_url);
        let body = serde_json::to_string(block).unwrap();

        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap();

        isahc::send(request).unwrap();
    }
}
