use std::{fmt, str::FromStr};

use generic_array::typenum::U32;
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::{
    digest::generic_array::{self, GenericArray},
    Sha3_256,
};
use thiserror::Error;

#[derive(Error, PartialEq, Eq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ConsensusHashError {
    #[error("Invalid format")]
    InvalidFormat,

    #[error("Invalid length")]
    InvalidLength,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct ConsensusHash([u8; 32]);

impl ConsensusHash {
    pub fn digest<T>(value: &T) -> Self
    where
        T: ?Sized + serde::Serialize,
    {
        let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
        let sha3_256_digest: GenericArray<u8, U32> = Sha3_256::digest(encoded);
        ConsensusHash(sha3_256_digest.into())
    }

    pub fn leading_zeros(&self) -> u32 {
        let mut count = 0;
        for byte in self.0 {
            let byte_leading_zeros = byte.leading_zeros();
            count += byte_leading_zeros;
            if byte_leading_zeros < 8 {
                break;
            }
        }

        count
    }
}

impl TryFrom<Vec<u8>> for ConsensusHash {
    type Error = ConsensusHashError;

    fn try_from(vec: Vec<u8>) -> Result<Self, ConsensusHashError> {
        let slice = vec.as_slice();
        match slice.try_into() {
            Ok(byte_array) => Ok(ConsensusHash(byte_array)),
            Err(_) => Err(ConsensusHashError::InvalidLength),
        }
    }
}

impl TryFrom<String> for ConsensusHash {
    type Error = ConsensusHashError;

    fn try_from(s: String) -> Result<Self, ConsensusHashError> {
        match hex::decode(s) {
            Ok(decoded_vec) => decoded_vec.try_into(),
            Err(_) => Err(ConsensusHashError::InvalidFormat),
        }
    }
}

impl FromStr for ConsensusHash {
    type Err = ConsensusHashError;

    fn from_str(s: &str) -> Result<Self, ConsensusHashError> {
        ConsensusHash::try_from(s.to_string())
    }
}

impl From<ConsensusHash> for String {
    fn from(hash: ConsensusHash) -> Self {
        hash.to_string()
    }
}

impl fmt::Display for ConsensusHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

pub trait ConsensusHashable {
    fn consensus_hash(&self) -> ConsensusHash;
}

impl<T: ?Sized + Serialize> ConsensusHashable for T {
    fn consensus_hash(&self) -> ConsensusHash {
        ConsensusHash::digest(self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::hash::{ConsensusHash, ConsensusHashError};

    #[test]
    fn no_leading_zeros() {
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        assert_leading_zeros(hex_str, 0);
    }

    #[test]
    fn leading_zeros_only_first_byte() {
        let hex_str = "0080b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        assert_leading_zeros(hex_str, 8);
    }

    #[test]
    fn leading_zeros_less_than_one_byte() {
        let hex_str = "0380b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        assert_leading_zeros(hex_str, 6);
    }

    #[test]
    fn full_leading_zeros() {
        let hex_str = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_leading_zeros(hex_str, 256);
    }

    fn assert_leading_zeros(hex_str: &str, leading_zeros: u32) {
        let hash = ConsensusHash::try_from(hex_str.to_string()).unwrap();
        assert_eq!(hash.leading_zeros(), leading_zeros);
    }

    #[test]
    fn parse_valid_hash() {
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e";
        let hash = ConsensusHash::try_from(hex_str.to_string()).unwrap();
        assert_eq!(hash.to_string(), hex_str);
        let hash = ConsensusHash::from_str(hex_str).unwrap();
        assert_eq!(hash.to_string(), hex_str);
    }

    #[test]
    fn parse_case_insensitive() {
        let hex_str =
            "F780B958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string();
        let hash = ConsensusHash::try_from(hex_str.clone()).unwrap();
        assert_eq!(hash.to_string(), hex_str.to_lowercase());
    }

    #[test]
    fn parse_json() {
        let hex_str =
            "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string();
        let hash: ConsensusHash =
            serde_json::from_value(serde_json::Value::String(hex_str.clone())).unwrap();
        assert_eq!(hash.to_string(), hex_str.to_lowercase());
        let hash_json = serde_json::to_value(hash).unwrap();
        assert_eq!(hash_json, serde_json::Value::String(hex_str.clone()));
    }

    #[test]
    fn reject_too_short() {
        // 31-byte string (62 hex chars)
        let hex_str = "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce2".to_string();
        let err = ConsensusHash::try_from(hex_str).unwrap_err();
        assert_eq!(err, ConsensusHashError::InvalidLength);
    }

    #[test]
    fn reject_too_long() {
        // 33-byte string (66 hex chars)
        let hex_str =
            "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e10".to_string();
        let err = ConsensusHash::try_from(hex_str).unwrap_err();
        assert_eq!(err, ConsensusHashError::InvalidLength);
    }

    #[test]
    fn reject_invalid_characters() {
        // correct length (32 bytes) but with an invalid hexadecimal char "g"
        let hex_str =
            "g780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string();
        let err = ConsensusHash::try_from(hex_str).unwrap_err();
        assert_eq!(err, ConsensusHashError::InvalidFormat);
    }
}
