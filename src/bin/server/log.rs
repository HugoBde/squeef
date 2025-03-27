use std::fmt::Debug;
use std::io::{Result, Write};

use chrono::Local;

pub struct Logger {
    name: String,
    log_level: LogLevel,
    output: Box<dyn Write + Send>,
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
    pub fn new(name: String, log_level: LogLevel, output: Box<dyn Write + Send>) -> Logger {
        Logger {
            name,
            log_level,
            output,
        }
    }
}

impl Write for Logger {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        return self.output.write(buf);
    }

    fn flush(&mut self) -> Result<()> {
        return self.output.flush();
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

#[derive(Debug)]
pub struct Loggers(Vec<Logger>);

impl Loggers {
    pub fn log(&mut self, log_level: LogLevel, msg: &str) -> () {
        let t = Local::now().format("[%Y-%m-%d %H:%M:%S]");
        let prefix = match log_level {
            LogLevel::DEBUG => DEBUG_PREFIX,
            LogLevel::INFO => INFO_PREFIX,
            LogLevel::WARN => WARN_PREFIX,
            LogLevel::ERROR => ERR_PREFIX,
        };

        let s = format!("{}{} {}\n", prefix, t, msg);

        for l in &mut self.0 {
            if l.log_level > log_level {
                continue;
            }

            l.write(s.as_bytes()).unwrap();
        }
    }
}

impl From<Vec<Logger>> for Loggers {
    fn from(v: Vec<Logger>) -> Loggers {
        Loggers(v)
    }
}
