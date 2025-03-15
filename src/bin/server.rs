use std::io::stderr;

use squeef::log::{LogLevel, Logger};
use squeef::server::Server;

fn main() {
    let logger = Logger::new(String::from("default"), LogLevel::DEBUG, Box::new(stderr()));

    let mut s = Server::new(6870, vec![logger]);

    s.run();
}
