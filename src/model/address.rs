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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
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

impl TryFrom<String> for Address {
    type Error = AddressError;

    fn try_from(s: String) -> Result<Self, AddressError> {
        match hex::decode(s) {
            Ok(decoded_vec) => decoded_vec.try_into(),
            Err(_) => Err(AddressError::InvalidFormat),
        }
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, AddressError> {
        Address::try_from(s.to_string())
    }
}

impl From<Address> for String {
    fn from(account: Address) -> Self {
        account.to_string()
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

// Some sample addresses to be used in tests all over the project
// We export functions to workaround constant value restrictions in Rust
#[cfg(test)]
pub mod test_util {
    use std::convert::TryFrom;

    use super::Address;

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

    pub fn carol() -> Address {
        Address::try_from(
            "b4f8293fb123ef3ff9ad49e923f4afc732774ee2bfdc3b278a359b54473c2277".to_string(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, str::FromStr};

    use crate::model::Address;

    use super::AddressError;

    #[test]
    fn parse_valid_address() {
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        let address = Address::try_from(hex_str.to_string()).unwrap();
        assert_eq!(address.to_string(), hex_str);
        let address = Address::from_str(hex_str).unwrap();
        assert_eq!(address.to_string(), hex_str);
    }

    #[test]
    fn parse_case_insensitive() {
        let hex_str =
            "F780B958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string();
        let address = Address::try_from(hex_str.clone()).unwrap();
        assert_eq!(address.to_string(), hex_str.to_lowercase());
    }

    #[test]
    fn parse_json() {
        let hex_str =
            "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string();
        let address: Address =
            serde_json::from_value(serde_json::Value::String(hex_str.clone())).unwrap();
        assert_eq!(address.to_string(), hex_str.to_lowercase());
        let address_json = serde_json::to_value(address).unwrap();
        assert_eq!(address_json, serde_json::Value::String(hex_str.clone()));
    }

    #[test]
    fn reject_too_short() {
        // 31-byte string (62 hex chars)
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce2".to_string();
        let err = Address::try_from(hex_str).unwrap_err();
        assert_eq!(err, AddressError::InvalidLength);
    }

    #[test]
    fn reject_too_long() {
        // 33-byte string (66 hex chars)
        let hex_str =
            "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e10".to_string();
        let err = Address::try_from(hex_str).unwrap_err();
        assert_eq!(err, AddressError::InvalidLength);
    }

    #[test]
    fn reject_invalid_characters() {
        // correct length (32 bytes) but with an invalid hexadecimal char "g"
        let hex_str =
            "g780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string();
        let err = Address::try_from(hex_str).unwrap_err();
        assert_eq!(err, AddressError::InvalidFormat);
    }
}
