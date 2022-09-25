use isahc::{ReadResponseExt, Request};
use spec::types::Block;

pub trait NodeClient {
    fn get_block_template(&self) -> Block;
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
    fn get_block_template(&self) -> Block {
        let uri = format!("{}/block_template", self.node_url);
        let mut response = isahc::get(uri).unwrap();

        // check that the response is sucessful
        assert_eq!(response.status().as_u16(), 200);

        // parse and return block template
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
