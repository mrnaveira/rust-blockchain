extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::str::FromStr;

// Encapsulates configuration values to be used across the application
// It ensures correct typing and that at least they will have a default value
pub struct Config {
    // Networking settings
    pub port: u16,

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
            port: Config::read_envvar::<u16>("PORT", 8000),

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_present_envvar() {
        let var_name = "PRESENT_ENVVAR";
        let real_value = 9000;
        env::set_var(var_name, real_value.to_string());

        // read the present, should NOT return the default value but the real one
        let default_value = 8000 as u16;
        let value = Config::read_envvar::<u16>(var_name, default_value);

        assert_eq!(value, real_value);

        // let's remove the var at the end to not pollute the environment
        env::remove_var(var_name);
    }

    #[test]
    fn read_non_present_envvar() {
        let var_name = "NON_PRESENT_ENVVAR";

        // let's remove the var just to make sure it's not setted
        env::remove_var(var_name);

        // read the non present var, should return the default value
        let default_value = 8000 as u16;
        let value = Config::read_envvar::<u16>(var_name, default_value);

        assert_eq!(value, default_value);
    }

    #[test]
    fn read_invalid_envvar() {
        // envvars should not have the "=" character in the name
        let var_name = "INVALID=VAR=NAME";

        // read the invalid var, should return the default value
        let default_value = 8000 as u16;
        let value = Config::read_envvar::<u16>(var_name, default_value);

        assert_eq!(value, default_value);
    }
}
