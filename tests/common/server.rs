use std::{
    convert::TryInto,
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
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

#[allow(dead_code)]
impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        // set the default values
        let config = Config {
            port: 8000,
            // not to high to avoid waiting too much, not too shot to spam it
            peer_sync_ms: 10,
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

type SyncedOutput = Arc<Mutex<Vec<String>>>;

pub struct Server {
    pub config: Config,
    process: Child,
    output: SyncedOutput,
}

#[allow(dead_code)]
impl Server {
    pub fn new(config: Config) -> Server {
        let mut process = Server::start_process(&config);
        let output = Server::start_stdout_reading(&mut process);

        let mut server = Server {
            process,
            config,
            output,
        };

        // We return the server only after all the processes have started
        // The last process to start is the rest api, so we wait until de output indicates it
        server.wait_for_log_message("actix-web-service");

        server
    }

    // start the blockchain application in the background
    fn start_process(config: &Config) -> Child {
        Command::new(cargo_bin("rust_blockchain"))
            .env("PORT", config.port.to_string())
            .env("PEERS", config.peers.join(","))
            .env("DIFFICULTY", config.difficulty.to_string())
            .env("TRANSACTION_WAITING_MS", config.tx_waiting_ms.to_string())
            .env("PEER_SYNC_MS", config.peer_sync_ms.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    }

    // start reading the process output in a separate thread (to not block the execution)
    // and continously update a shared value ("output") containing all the log messages in order
    fn start_stdout_reading(process: &mut Child) -> SyncedOutput {
        let output = Arc::new(Mutex::new(Vec::<String>::new()));
        let thread_output = output.clone();
        let stdout = process.stdout.take().unwrap();
        thread::spawn(move || {
            let buf = BufReader::new(stdout);
            for line in buf.lines() {
                match line {
                    Ok(_) => {
                        thread_output.lock().unwrap().push(line.unwrap());
                        continue;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        output
    }

    // block the execution until we mine a new block
    pub fn wait_for_mining(&mut self) {
        self.wait_for_log_message("valid block found for index");
    }

    // block the execution until we sync a new block
    pub fn wait_for_peer_sync(&mut self) {
        self.wait_for_log_message("Added new peer block");
    }

    // block the execution until we receive a new block via api
    pub fn wait_to_receive_block_in_api(&mut self) {
        self.wait_for_log_message("Received new block");
    }

    // block the execution until a message is contained in the process output
    // or until a max time has passed
    fn wait_for_log_message(&mut self, message: &str) {
        // time interval to check for new output messages
        let wait_time = Duration::from_millis(50);
        // max time that we are going to wait for the message to appear
        let max_wait_time = Duration::from_millis(500);

        let start = Instant::now();
        while Instant::now() < start + max_wait_time {
            let message_was_found = self.search_message_in_output(message);
            if message_was_found {
                return;
            }
            thread::sleep(wait_time);
        }
    }

    fn search_message_in_output(&mut self, message: &str) -> bool {
        let lines = self.output.lock().unwrap();
        for line in lines.iter() {
            if line.contains(message) {
                return true;
            }
        }

        false
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

    fn sleep_millis(millis: u64) {
        let wait_duration = Duration::from_millis(millis);
        thread::sleep(wait_duration);
    }
}

// Stopping the server on variable drop allows us to never worry about
// leaving a zombie child process in the background.
// The Rust compiler ensures that this will be always called no matter what (success or panic)
// as soon as the variable is out of scope
impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}
