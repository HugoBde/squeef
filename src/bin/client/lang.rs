use std::{error::Error, fmt::Display};

use squeef::command::Command;

fn tokenize(command: &str) -> Vec<&str> {
    return command.split_whitespace().collect();
}

pub fn parse(user_input: String) -> Result<Command, ParseError> {
    let tokens = tokenize(&user_input);

    return match tokens[0] {
        "CREATE" => match tokens[1] {
            "DATABASE" | "DB" => Ok(Command::CreateDatabase {
                name: String::from(tokens[2]),
            }),
            "TABLE" => Ok(Command::CreateTable {
                name: String::from(tokens[2]),
                cols: vec![],
            }),
            _ => Err(ParseError::InvalidCommand),
        },
        "OPEN" => Ok(Command::OpenDatabase {
            name: String::from(tokens[2]),
        }),
        "LIST" => match tokens[1] {
            "DATABASES" | "DBS" => Ok(Command::ListDatabases),
            "TABLES" => Ok(Command::ListTables),
            _ => Err(ParseError::InvalidCommand),
        },
        _ => Err(ParseError::InvalidCommand),
    };
}

#[derive(Debug)]
pub enum ParseError {
    InvalidCommand,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            ParseError::InvalidCommand => write!(f, "Invalid command"),
        };
    }
}
