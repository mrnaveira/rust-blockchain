use super::Config;
use crate::model::{Blockchain, TransactionPool};

pub struct Context {
    pub config: Config,
    pub blockchain: Blockchain,
    pub pool: TransactionPool,
}
