use spec::types::Network;

use crate::{
    api::Api,
    database::ConcurrentNodeDatabase,
    peer::Peer,
    util::{config::Config, execution},
};

pub struct Server {
    pub config: Config,
    pub database: ConcurrentNodeDatabase,
}

impl Server {
    pub fn new(config: Config) -> Self {
        // TODO: read the network definition from a file
        let network = Network {
            description: "Test network".to_string(),
            difficulty: 0,
            timestamp: 0,
        };

        let database = ConcurrentNodeDatabase::new(network);

        Self { config, database }
    }

    pub fn start(&self) {
        let api = Api::new(self.config.port, &self.database.clone());
        let peer = Peer::new(&self.config, &self.database.clone());
        execution::run_in_parallel(vec![&api, &peer]);
    }
}
