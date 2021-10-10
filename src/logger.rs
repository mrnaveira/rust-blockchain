use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn init() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter(None, LevelFilter::Info);
    builder.init();
}
