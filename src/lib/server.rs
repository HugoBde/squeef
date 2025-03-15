use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};

use crate::database::Database;
use crate::log::{LogLevel, Logger};
use crate::protocol::v0::{self, Command};
use crate::table::Table;
use crate::utils;

#[derive(Debug)]
pub struct Server {
    port: u16,
    loggers: Vec<Logger>,
    databases: Vec<Database>,
    open_db: Option<usize>,
}

impl Server {
    pub fn new(port: u16, loggers: Vec<Logger>) -> Server {
        Server {
            port,
            loggers,
            databases: vec![],
            open_db: None,
        }
    }

    pub fn run(&mut self) -> () {
        self.log(
            LogLevel::INFO,
            &format!("Started listening on {}", self.port),
        );

        let listener = TcpListener::bind(("127.0.0.1", self.port)).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_client(stream),
                Err(e) => {
                    self.log(LogLevel::ERROR, &e.to_string());
                    return;
                }
            }
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) -> () {
        loop {
            match utils::read_msg(&mut stream) {
                Ok(msg) => {
                    if let Err(e) = self.process_msg(msg.as_slice()) {
                        self.log(LogLevel::ERROR, &e.to_string());
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::UnexpectedEof => {
                        self.log(LogLevel::INFO, "Connection closed");
                        return;
                    }
                    _ => {
                        self.log(LogLevel::ERROR, &e.to_string());
                        return;
                    }
                },
            }
        }
    }

    fn process_msg(&mut self, msg: &[u8]) -> Result<(), String> {
        if msg.len() == 0 {
            return Err(String::from("Invalid message: incomplete header"));
        }

        let cmd = v0::Command::try_from(msg)?;

        match cmd {
            Command::CreateDatabase { name } => self.exec_create_db(name),
            Command::OpenDatabase { name } => self.exec_open_db(name),
            Command::CreateTable { name, .. } => self.exec_create_table(name),
            Command::Dump => self.exec_dump(),
        }
    }

    fn exec_create_db(&mut self, name: String) -> Result<(), String> {
        if self.databases.iter().find(|db| db.name == name).is_some() {
            return Err(format!(
                "Failed to create database. Name [{}] already in use",
                name
            ));
        }

        let new_db = Database::new(name.clone());

        self.databases.push(new_db);

        self.log(LogLevel::INFO, &format!("Created database [{}]", name));

        return Ok(());
    }

    fn exec_open_db(&mut self, name: String) -> Result<(), String> {
        let pos = self.databases.iter().position(|db| db.name == name);

        if pos.is_none() {
            return Err(format!(
                "Failed to open database. No database with name [{}]",
                name
            ));
        }

        self.open_db = pos;

        self.log(LogLevel::DEBUG, &format!("Opened database [{}]", name));

        return Ok(());
    }

    fn exec_create_table(&mut self, name: String) -> Result<(), String> {
        if self.open_db.is_none() {
            return Err(format!("CREATE TABLE failed. No open database"));
        }

        let open_db = &mut self.databases[self.open_db.unwrap()];

        let tables = &mut open_db.tables;

        if tables.iter().find(|tb| tb.name == name).is_some() {
            return Err(format!(
                "CREATE TABLE failed. Name [{}::{}] already in use",
                open_db.name, name
            ));
        }

        tables.push(Table::new(name.clone()));

        // Shadow old mut ref to open_db with a regular ref to open_db
        let open_db = &self.databases[self.open_db.unwrap()];

        self.log(
            LogLevel::INFO,
            &format!("Created table [{}] in database [{}]", name, open_db.name),
        );

        return Ok(());
    }

    fn exec_dump(&self) -> Result<(), String> {
        if self.open_db.is_none() {
            return Err(format!("DUMP failed. No open database"));
        }

        let open_db = &self.databases[self.open_db.unwrap()];

        eprintln!("DUMP: {open_db:#?}");

        return Ok(());
    }

    fn log(&mut self, log_level: LogLevel, msg: &str) {
        self.loggers
            .iter_mut()
            .for_each(|l| l.log(log_level, msg).unwrap());
    }
}
