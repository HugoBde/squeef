// Std Lib Imports
use std::io::stderr;

// Executable Imports
mod config;
mod log;
mod server;
mod thread_pool;

use config::CONFIG;
use log::{LogLevel, Logger, Loggers};
use server::Server;

fn main() {
    let loggers = Loggers::from(vec![Logger::new(
        String::from("default"),
        LogLevel::DEBUG,
        Box::new(stderr()),
    )]);

    let mut s = Server::new(CONFIG.server.port, loggers);

    s.run();
}
