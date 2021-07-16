extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::str::FromStr;

type StringVec = Vec<String>;

pub struct Config {
    // Networking settings
    pub client_port: u16,
    pub peer_port: u16,
    pub peers: StringVec,

    // Miner settings
    pub max_nonce: u64,
    pub difficulty: usize,
    pub tx_waiting_seconds: u64
}

impl Config {
    pub fn read() -> Config {
        dotenv().ok();

        let config = Config {
            // Networking settings
            client_port: Config::read_envvar::<u16>("CLIENT_PORT", 8000),
            peer_port:  Config::read_envvar::<u16>("PEER_PORT", 9000),
            peers: Config::read_vec_envvar("PEERS", ",", StringVec::default()),

            // Miner settings
            max_nonce: Config::read_envvar::<u64>("MAX_NONCE", 1_000_000),
            difficulty: Config::read_envvar::<usize>("DIFFICULTY", 10),
            tx_waiting_seconds: Config::read_envvar::<u64>("TRANSACTION_WAITING_SECONDS", 10),
        };
        
        return config;
    }

    fn read_envvar<T: FromStr>(key: &str, default_value: T) -> T {
        match env::var(key) {
            Ok(val) => return val.parse::<T>().unwrap_or(default_value),
            Err(_e) => return default_value
        }
    }

    fn read_vec_envvar(key: &str, separator: &str, default_value: StringVec) -> StringVec {
        match env::var(key) {
            Ok(val) => return val.trim().split(separator).map(str::to_string).collect(),
            Err(_e) => return default_value
        }
    }
}