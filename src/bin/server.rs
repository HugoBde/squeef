use std::cell::RefCell;
use std::io::{self, stderr, BufRead, BufReader, ErrorKind, Read};
use std::net::{TcpListener, TcpStream};

use squeef::database::Database;
use squeef::log::Logger;
use squeef::server::Server;

fn main() {
    let logger = Logger::new(String::from("default"), Box::new(stderr()));

    let mut s = Server::new(6870, vec![logger]);

    s.run();
}
