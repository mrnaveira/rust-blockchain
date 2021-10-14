use crate::config::Config;
use crate::model::{Blockchain, TransactionPool};

pub struct SharedData {
    pub config: Config,
    pub blockchain: Blockchain,
    pub pool: TransactionPool,
}
