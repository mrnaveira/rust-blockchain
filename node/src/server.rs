use crate::{
    api::Api,
    database::Database,
    peer::Peer,
    util::{execution, Config},
};

pub struct Server {
    config: Config,
    database: Database,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let database = Database::new(&config);

        Self { config, database }
    }

    pub fn start(&self) {
        let api = Api::new(self.config.port, &self.database);
        let peer = Peer::new(&self.config, &self.database);
        execution::run_in_parallel(vec![&api, &peer]);
    }
}
