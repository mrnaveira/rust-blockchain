#[macro_use]
extern crate log;

mod api;
mod database;
mod peer;
mod server;
mod util;

use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::{server::Server, util::Config};

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
    set_ctrlc_handler();
}

fn initialize_logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter(None, LevelFilter::Info);
    builder.init();
}

pub fn set_ctrlc_handler() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
