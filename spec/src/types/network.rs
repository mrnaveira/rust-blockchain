use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Network {
    pub description: String,
    pub difficulty: u32,
    pub timestamp: i64,
}
