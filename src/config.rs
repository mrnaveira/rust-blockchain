extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::str::FromStr;

type StringVec = Vec<String>;

pub struct Config {
    pub client_port: u16,
    pub peer_port: u16,
    pub peers: StringVec,
}

impl Config {
    pub fn read() -> Config {
        dotenv().ok();

        let config = Config {
            client_port: Config::read_envvar::<u16>("CLIENT_PORT", 8000),
            peer_port:  Config::read_envvar::<u16>("PEER_PORT", 9000),
            peers: Config::read_vec_envvar("PEERS", ",", StringVec::default()),
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