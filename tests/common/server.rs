use std::{
    convert::TryInto,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use assert_cmd::cargo::cargo_bin;
use nix::{
    sys::signal::{kill, Signal::SIGTERM},
    unistd::Pid,
};
pub struct Config {
    pub port: u16,
    pub peers: Vec<String>,
    pub peer_sync_ms: u64,
    pub max_blocks: u64,
    pub max_nonce: u64,
    pub difficulty: u32,
    pub tx_waiting_ms: u64,
}

pub struct ServerBuilder {
    config: Config,
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        // set the default values
        let config = Config {
            port: 8000,
            // not to high to avoid waiting too much, not too shot to spam it
            peer_sync_ms: 100,
            // no difficulty to minimize the mining time
            difficulty: 0,
            // not to high to avoid waiting, not too shot to spam it
            tx_waiting_ms: 10,
            peers: Vec::<String>::new(),
            max_blocks: 0, // unlimited blocks
            max_nonce: 0,  // unlimited nonce
        };

        ServerBuilder { config }
    }

    pub fn difficulty(mut self, difficulty: u32) -> ServerBuilder {
        self.config.difficulty = difficulty;
        self
    }

    pub fn port(mut self, port: u16) -> ServerBuilder {
        self.config.port = port;
        self
    }

    pub fn peer(mut self, port: u64) -> ServerBuilder {
        let address = format!("http://localhost:{}", port);
        self.config.peers.push(address);
        self
    }

    pub fn start(self) -> Server {
        Server::new(self.config)
    }
}

pub struct Server {
    pub config: Config,
    process: Child,
}

impl Server {
    pub fn new(config: Config) -> Server {
        Server {
            process: Server::start_process(&config),
            config: config,
        }
    }

    fn start_process(config: &Config) -> Child {
        // start the blockchain in the background
        let process = Command::new(cargo_bin("rust_blockchain"))
            .env("PORT", config.port.to_string())
            .env("PEERS", config.peers.join(","))
            .env("DIFFICULTY", config.difficulty.to_string())
            .env("TRANSACTION_WAITING_MS", config.tx_waiting_ms.to_string())
            .env("PEER_SYNC_MS", config.peer_sync_ms.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        // allow time for the blockchain to start
        Server::sleep_millis(1000);

        process
    }

    pub fn wait_for_mining(&self) {
        // wait a bit longer that the waiting time
        Server::sleep_millis(self.config.tx_waiting_ms * 10);
    }

    pub fn wait_for_peer_sync(&self) {
        // wait a bit longer that the waiting time
        Server::sleep_millis(self.config.peer_sync_ms * 10);
    }

    fn sleep_millis(millis: u64) {
        let wait_duration = Duration::from_millis(millis);
        thread::sleep(wait_duration);
    }

    fn stop(&mut self) {
        println!("Shutting down server on port {}", self.config.port);

        kill(self.get_pid(), SIGTERM).unwrap();

        // block the thread until the server has finished
        self.wait_for_termination();
    }

    fn get_pid(&mut self) -> Pid {
        Pid::from_raw(self.process.id().try_into().unwrap())
    }

    fn wait_for_termination(&mut self) {
        let max_waiting_in_secs = 5;

        // check every second if the child has finished
        for _ in 0..max_waiting_in_secs {
            match self.process.try_wait().unwrap() {
                // has finished, so we exit
                Some(_) => return,
                // hasn't finished, we wait another second
                None => Server::sleep_millis(1000),
            }
        }

        // at this point, we waited but the child didn't finish
        // so we forcefully kill it
        let _ = self.process.kill();
        self.process.wait().unwrap();
    }
}

// Stopping the server on variable drop allows us to never worry about
// leaving a zombie child process in the background.
// The Rust compilers ensures that this will be always called no matter what (success or panic)
// as soon as the variable is out of scope
impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}
