// Third Party Imports
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(short, long, default_value = "6870")]
    pub port: u16,
}
