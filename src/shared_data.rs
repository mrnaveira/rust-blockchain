use crate::blockchain::Blockchain;
use crate::config::Config;
use crate::transaction_pool::TransactionPool;

pub struct SharedData {
    pub config: Config,
    pub blockchain: Blockchain,
    pub pool: TransactionPool,
}
