use std::fmt::Debug;
use std::io::{Result, Write};

pub struct Logger {
    name:   String,
    output: Box<dyn Write>,
}

// const RED_FG : &str = "\x1b[31m";
// const GREEN_FG : &str = "\x1b[32m";
// const YELLOW_FG : &str = "\x1b[33m";
// const RESET : &str = "\x1b[0m";

const INFO_PREFIX: &str = "[\x1b[32mINFO\x1b[0m]";
const WARN_PREFIX: &str = "[\x1b[33mWARN\x1b[0m]";
const ERR_PREFIX: &str = "[\x1b[31mERR\x1b[0m] ";

impl Logger {
    pub fn new(name: String, output: Box<dyn Write>) -> Logger {
        Logger {
            name,
            output,
        }
    }

    pub fn info(&mut self, msg: &String) -> Result<()> {
        return writeln!(self.output, "{} :: {}", INFO_PREFIX, msg);
    }

    pub fn warn(&mut self, msg: &String) -> Result<()> {
        return writeln!(self.output, "{} :: {}", WARN_PREFIX, msg);
    }

    pub fn error(&mut self, msg: &String) -> Result<()> {
        return writeln!(self.output, "{} :: {}", ERR_PREFIX, msg);
    }
}

impl Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger[{}]", self.name)
    }
}
