use std::io::stderr;

mod config;
mod log;
mod server;

use config::CONFIG;
use log::{LogLevel, Logger};
use server::Server;

fn main() {
    println!("{:#?}", *CONFIG);

    let logger = Logger::new(String::from("default"), LogLevel::DEBUG, Box::new(stderr()));

    let mut s = Server::new(CONFIG.server.port, vec![logger]);

    s.run();
}
