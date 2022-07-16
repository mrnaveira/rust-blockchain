use serde::{Deserialize, Serialize};

use super::Address;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub sender: Address,
    pub recipient: Address,
    pub amount: u64,
}
