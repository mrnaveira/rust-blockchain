use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

// Addresses are 32-bytes long
type Byte = u8;
const LEN: usize = 32;

#[derive(Error, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AddressError {
    #[error("Invalid format")]
    InvalidFormat,

    #[error("Invalid length")]
    InvalidLength,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address([Byte; LEN]);

impl TryFrom<Vec<Byte>> for Address {
    type Error = AddressError;

    fn try_from(vec: Vec<Byte>) -> Result<Self, AddressError> {
        let slice = vec.as_slice();
        match slice.try_into() {
            Ok(byte_array) => Ok(Address(byte_array)),
            Err(_) => Err(AddressError::InvalidLength),
        }
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match hex::decode(s) {
            Ok(decoded_vec) => decoded_vec.try_into(),
            Err(_) => Err(AddressError::InvalidFormat),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::model::Address;

    use super::AddressError;

    #[test]
    fn parse_valid_address() {
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        let address = Address::from_str(hex_str).unwrap();
        assert_eq!(address.to_string(), hex_str);
    }

    #[test]
    fn parse_case_insensitive() {
        let hex_str = "F780B958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        let address = Address::from_str(hex_str).unwrap();
        assert_eq!(address.to_string(), hex_str.to_lowercase());
    }

    #[test]
    fn reject_too_short() {
        // 31-byte string (62 hex chars)
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce2";
        let err = Address::from_str(hex_str).unwrap_err();
        assert_eq!(err, AddressError::InvalidLength);
    }

    #[test]
    fn reject_too_long() {
        // 33-byte string (66 hex chars)
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e10";
        let err = Address::from_str(hex_str).unwrap_err();
        assert_eq!(err, AddressError::InvalidLength);
    }

    #[test]
    fn reject_invalid_characters() {
        // correct length (32 bytes) but with an invalid hexadecimal char "g"
        let hex_str = "g780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        let err = Address::from_str(hex_str).unwrap_err();
        assert_eq!(err, AddressError::InvalidFormat);
    }
}
