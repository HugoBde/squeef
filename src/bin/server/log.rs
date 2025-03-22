use std::fmt::Debug;
use std::io::{Result, Write};

use chrono::Local;

pub struct Logger {
    name: String,
    log_level: LogLevel,
    output: Box<dyn Write>,
}

// const RED_FG : &str = "\x1b[31m";
// const GREEN_FG : &str = "\x1b[32m";
// const YELLOW_FG : &str = "\x1b[33m";
// const RESET : &str = "\x1b[0m";

const INFO_PREFIX: &str = "[\x1b[32mINFO\x1b[0m]";
const DEBUG_PREFIX: &str = "[\x1b[34mDBUG\x1b[0m]";
const WARN_PREFIX: &str = "[\x1b[33mWARN\x1b[0m]";
const ERR_PREFIX: &str = "[\x1b[31mERRR\x1b[0m]";

impl Logger {
    pub fn new(name: String, log_level: LogLevel, output: Box<dyn Write>) -> Logger {
        Logger {
            name,
            log_level,
            output,
        }
    }

    pub fn log(&mut self, log_level: LogLevel, msg: &str) -> Result<()> {
        if self.log_level > log_level {
            return Ok(());
        }
        let t = Local::now().format("[%Y-%m-%d %H:%M:%S]");
        let prefix = match log_level {
            LogLevel::DEBUG => DEBUG_PREFIX,
            LogLevel::INFO => INFO_PREFIX,
            LogLevel::WARN => WARN_PREFIX,
            LogLevel::ERROR => ERR_PREFIX,
        };
        return writeln!(self.output, "{}{} {}", prefix, t, msg);
    }
}

impl Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger[{}]", self.name)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    DEBUG = 0x00,
    INFO = 0x01,
    WARN = 0x02,
    ERROR = 0x03,
}
