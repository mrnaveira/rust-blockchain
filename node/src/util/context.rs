use spec::Blockchain;

use crate::transaction_pool::TransactionPool;

use super::Config;

pub struct Context {
    pub config: Config,
    pub blockchain: Blockchain,
    pub pool: TransactionPool,
}
