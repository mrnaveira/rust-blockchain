#[macro_use]
extern crate log;

mod api;
mod database;
mod mempool;
mod peer;
mod server;
mod util;

use crate::{
    server::Server,
    util::{initialize_logger, termination, Config},
};

fn main() {
    // set up the logging system
    initialize_logger();
    info!("starting up");

    // read the configuration file
    let config = Config::read();

    // run the server
    let server = Server::new(config);
    server.start();

    // when user inputs Ctrl-C, terminate the program
    termination::set_ctrlc_handler();
}
