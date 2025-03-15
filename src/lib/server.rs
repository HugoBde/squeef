use std::io::{BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream};

use crate::database::Database;
use crate::log::Logger;
use crate::protocol::v0::{self, Command};
use crate::utils;

#[derive(Debug)]
pub struct Server {
    port:      u16,
    loggers:   Vec<Logger>,
    databases: Vec<Database>,
    open_db:   Option<usize>,
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
        self.log_info(format!("Started listening on {}", self.port));

        let listener = TcpListener::bind(("127.0.0.1", self.port)).unwrap();


        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_client(stream),
                Err(e) => {
                    self.log_error(e.to_string());
                    return;
                }
            }
        }
    }

    fn handle_client(&mut self, stream: TcpStream) -> () {
        let mut reader = BufReader::new(stream);

        loop {
            match utils::read_msg(&mut reader) {
                Ok(msg) => {
                    println!("Received msg >> {:x?}", msg);
                    if let Err(e) = self.process_msg(msg.as_slice()) {
                        self.log_error(e.to_string());
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::UnexpectedEof => {
                        self.log_info(String::from("Connection closed"));
                        return;
                    }
                    _ => {
                        self.log_error(e.to_string());
                        return;
                    }
                },
            }
        }
    }

    fn process_msg(&mut self, msg: &[u8]) -> Result<(), String> {
        if msg.len() < 2 {
            return Err(String::from("Invalid message: incomplete header"));
        }

        let cmd = v0::Command::try_from(msg)?;

        match cmd {
            Command::CreateDatabase {
                name,
            } => self.exec_create_db(name),
            Command::OpenDatabase {
                name,
            } => self.exec_open_db(name),
            Command::CreateTable {
                name, ..
            } => self.exec_create_table(name),
            Command::Dump => self.exec_dump(),
        }
    }

    fn exec_create_db(&mut self, name: String) -> Result<(), String> {
        if self.databases.iter().find(|db| db.name == name).is_some() {
            return Err(format!(
                "CREATE DB failed. Name [{}] already in use",
                name
            ));
        }

        self.databases.push(Database::new(name));

        return Ok(());
    }

    fn exec_open_db(&mut self, name: String) -> Result<(), String> {
        let pos = self.databases.iter().position(|db| db.name == name);

        if pos.is_none() {
            return Err(format!(
                "OPEN DB failed. No database with name [{}]",
                name
            ));
        }

        self.open_db = pos;

        return Ok(());
    }

    fn exec_create_table(&mut self, name: String) -> Result<(), String> {
        if self.open_db.is_none() {
            return Err(format!("CREATE TABLE failed. No open database"));
        }

        let open_db = &mut self.databases[self.open_db.unwrap()];

        if open_db.tables.iter().find(|tb| tb.name == name).is_some() {
            return Err(format!(
                "CREATE TABLE failed. Name [{}::{}] already in use",
                open_db.name, name
            ));
        }

        self.databases.push(Database::new(name));

        return Ok(());
    }

    fn exec_dump(&self) -> Result<(), String> {
        eprintln!("DUMP: {self:?}");
        return Ok(());
    }

    fn log_info(&mut self, msg: String) {
        self.loggers.iter_mut().for_each(|l| l.info(&msg).unwrap());
    }

    fn log_error(&mut self, msg: String) {
        self.loggers.iter_mut().for_each(|l| l.error(&msg).unwrap());
    }
}
