extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::str::FromStr;

type StringVec = Vec<String>;

// Encapsulates configuration values to be used across the application
// It ensures correct typing and that at least they will have a default value
pub struct Config {
    // Networking settings
    pub port: u16,
    pub peers: StringVec,

    // Miner settings
    pub max_blocks: u64,
    pub max_nonce: u64,
    pub difficulty: usize,
    pub tx_waiting_ms: u64,
}

// The implementation reads the values from environment variables
// If a value is missing then it enforces a default value
impl Config {
    // Parse and return configuration values from environment variables
    pub fn read() -> Config {
        dotenv().ok();

        Config {
            // Networking settings
            port: Config::read_envvar::<u16>("CLIENT_PORT", 8000),
            peers: Config::read_vec_envvar("PEERS", ",", StringVec::default()),

            // Miner settings
            max_blocks: Config::read_envvar::<u64>("MAX_BLOCKS", 0), // unlimited blocks
            max_nonce: Config::read_envvar::<u64>("MAX_NONCE", 1_000_000),
            difficulty: Config::read_envvar::<usize>("DIFFICULTY", 10),
            tx_waiting_ms: Config::read_envvar::<u64>("TRANSACTION_WAITING_MS", 10000),
        }
    }

    // Parses a singular value from a environment variable, accepting a default value if missing
    fn read_envvar<T: FromStr>(key: &str, default_value: T) -> T {
        match env::var(key) {
            Ok(val) => val.parse::<T>().unwrap_or(default_value),
            Err(_e) => default_value,
        }
    }

    // Parses a multiple value (Vec) from a environment variable, accepting a default value if missing
    fn read_vec_envvar(key: &str, separator: &str, default_value: StringVec) -> StringVec {
        match env::var(key) {
            Ok(val) => val.trim().split(separator).map(str::to_string).collect(),
            Err(_e) => default_value,
        }
    }
}
