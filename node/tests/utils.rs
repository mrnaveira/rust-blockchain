use std::{thread, time::Duration};

use isahc::{Body, ReadResponseExt, Request, Response};
use node::{server::Server, util::config::Config};

use miner::{cli::MinerArgs, mining_loop::run_mining_loop, node_client::NetworkNodeClient};
use spec::{
    types::{Address, Block, Transaction},
    validators::BLOCK_SUBSIDY,
};

const DEFAULT_PORT: u16 = 8000;
const DEFAULT_DIFFICULTY: u32 = 0;

pub struct TestServer {
    pub config: Config,
}

#[allow(dead_code)]
impl TestServer {
    pub fn start(&self) {
        // thread::sleep(Duration::from_millis(1100));

        let server = Server::new(self.config.clone());
        let _handle = thread::spawn(move || {
            server.start();
        });

        // we need to allow time for the server to start
        // TODO: improve it by inspecting the server itself (logs, variables, etc)
        // or make start a future
        thread::sleep(Duration::from_millis(100));
    }

    pub fn wait_for_peer_sync(&self) {
        // TODO: improve it by inspecting the server itself (logs, variables, etc)
        thread::sleep(Duration::from_millis(self.config.peer_sync_ms + 100));
    }

    pub fn wait_to_receive_block_in_api(&self) {
        // TODO: improve it by inspecting the server itself (logs, variables, etc)
        thread::sleep(Duration::from_millis(self.config.peer_sync_ms + 100));
    }
}

pub struct TestServerBuilder {
    config: Config,
}

#[allow(dead_code)]
impl TestServerBuilder {
    pub fn new() -> TestServerBuilder {
        // set the default values
        let config = Config {
            port: DEFAULT_PORT,
            // not to high to avoid waiting too much, not too shot to spam it
            peer_sync_ms: 10,
            // no difficulty to minimize the mining time
            difficulty: DEFAULT_DIFFICULTY,
            peers: Vec::<String>::new(),
        };

        TestServerBuilder { config }
    }

    pub fn difficulty(mut self, difficulty: u32) -> TestServerBuilder {
        self.config.difficulty = difficulty;
        self
    }

    pub fn port(mut self, port: u16) -> TestServerBuilder {
        self.config.port = port;
        self
    }

    pub fn peer(mut self, port: u64) -> TestServerBuilder {
        let address = format!("http://localhost:{}", port);
        self.config.peers.push(address);
        self
    }

    pub fn build(self) -> TestServer {
        TestServer {
            config: self.config,
        }
    }
}

pub fn miner_address() -> Address {
    Address::try_from(
        "fe8aa8cc6011898d49bdacd0ab52075e92e1dfb2915bb9223528a5737583731d".to_string(),
    )
    .unwrap()
}

pub fn alice() -> Address {
    Address::try_from(
        "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string(),
    )
    .unwrap()
}

pub fn bob() -> Address {
    Address::try_from(
        "51df097c03c0a6e64e54a6fce90cb6968adebd85955917ed438e3d3c05f2f00f".to_string(),
    )
    .unwrap()
}

pub trait RestApi {
    fn get_base_url(&self) -> String;
    fn get_blocks(&self) -> Vec<Block>;
    fn get_last_block(&self) -> Block;
    fn add_block(&self, block: &Block) -> Response<Body>;
    fn add_valid_block(&self) -> Response<Body>;
    fn add_transaction(&self, transaction: &Transaction) -> Response<Body>;
}

impl RestApi for TestServer {
    fn get_base_url(&self) -> String {
        format!("http://localhost:{}", self.config.port)
    }

    fn get_blocks(&self) -> Vec<Block> {
        // list the blocks by querying the REST API
        let uri = format!("{}/blocks", self.get_base_url());
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
        let coinbase = Transaction {
            sender: alice(),
            recipient: bob(),
            amount: BLOCK_SUBSIDY,
        };

        let index = last_block.index + 1;
        let previous_hash = last_block.hash;
        let transactions = vec![coinbase];
        let valid_block = Block::new(index, 0, previous_hash, transactions);

        self.add_block(&valid_block)
    }

    fn add_block(&self, block: &Block) -> Response<Body> {
        // send the request to the REST API
        let uri = format!("{}/blocks", self.get_base_url());
        let body = serde_json::to_string(&block).unwrap();

        post_request(uri, body)
    }

    fn add_transaction(&self, transaction: &Transaction) -> Response<Body> {
        // send the request to the REST API
        let uri = format!("{}/transactions", self.get_base_url());
        let body = serde_json::to_string(&transaction).unwrap();

        post_request(uri, body)
    }
}

fn post_request(uri: String, body: String) -> Response<Body> {
    let request = Request::post(uri)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    isahc::send(request).unwrap()
}

pub struct Miner {
    config: MinerArgs,
}

#[allow(dead_code)]
impl Miner {
    pub fn new() -> Self {
        let config = Self::default_config();
        Self { config }
    }

    pub fn new_with_node(node: &TestServer) -> Self {
        let mut config = Self::default_config();
        config.node_url = Self::get_node_url(node.config.port);
        Self { config }
    }

    pub fn mine_blocks(&self, num_blocks: u64) {
        let mut config = self.config.clone();
        config.max_blocks = num_blocks;

        let node_client = NetworkNodeClient::new(config.node_url.clone());

        run_mining_loop(config, node_client);

        thread::sleep(Duration::from_millis(100));
    }

    fn default_config() -> MinerArgs {
        MinerArgs {
            miner_address: miner_address(),
            node_url: Self::get_node_url(DEFAULT_PORT),
            difficulty: DEFAULT_DIFFICULTY,
            max_blocks: 1_u64,
            max_nonce: 1_000_000,
        }
    }

    fn get_node_url(port: u16) -> String {
        format!("http://localhost:{}", port)
    }
}
