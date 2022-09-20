use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short = 'p', long, value_parser, default_value = "8000")]
    pub port: u16,

    #[clap(short = 's', long, value_parser, default_value = "10000")]
    pub peer_sync_ms: u64,

    #[clap(short = 'd', long, value_parser, default_value = "10")]
    pub difficulty: u32,

    #[clap(long, value_parser, multiple = true)]
    pub peers: Vec<String>,
}

pub fn parse_from_cli() -> Config {
    Config::parse()
}
