use std::io::{ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use squeef::command::Command;
use squeef::database::Database;
use squeef::protocol::v0;
use squeef::table::Table;
use squeef::utils;

use crate::log::{LogLevel, Loggers};

#[derive(Debug)]
pub struct Server {
    port: u16,

    databases: Arc<RwLock<Vec<Database>>>,

    loggers: Arc<Mutex<Loggers>>,
}

impl Server {
    pub fn new(port: u16, loggers: Loggers) -> Server {
        Server {
            port,
            loggers: Arc::new(Mutex::new(loggers)),
            databases: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn run(&mut self) -> () {
        self.loggers.lock().unwrap().log(
            LogLevel::INFO,
            &format!("Started listening on {}", self.port),
        );

        let listener = TcpListener::bind(("127.0.0.1", self.port)).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.loggers.lock().unwrap().log(
                        LogLevel::INFO,
                        &format!("[{}] Incoming connection", stream.peer_addr().unwrap()),
                    );
                    let mut client_connection =
                        ClientConnection::new(stream, self.databases.clone(), self.loggers.clone());

                    thread::spawn(move || client_connection.run());
                }
                Err(e) => {
                    self.loggers
                        .lock()
                        .unwrap()
                        .log(LogLevel::ERROR, &e.to_string());
                    return;
                }
            }
        }
    }
}

pub struct ClientConnection {
    stream: TcpStream,
    databases: Arc<RwLock<Vec<Database>>>,
    loggers: Arc<Mutex<Loggers>>,
    open_db: Option<usize>,
}

impl ClientConnection {
    fn new(
        stream: TcpStream,
        databases: Arc<RwLock<Vec<Database>>>,
        loggers: Arc<Mutex<Loggers>>,
    ) -> ClientConnection {
        ClientConnection {
            stream,
            databases,
            loggers,
            open_db: None,
        }
    }

    fn run(&mut self) -> () {
        loop {
            match utils::read_msg(&mut self.stream) {
                Ok(msg) => {
                    if let Err(e) = self.process_msg(msg.as_slice()) {
                        self.loggers
                            .lock()
                            .unwrap()
                            .log(LogLevel::ERROR, &e.to_string());
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::UnexpectedEof => {
                        self.loggers.lock().unwrap().log(
                            LogLevel::INFO,
                            &format!("[{}] Connection closed", self.stream.peer_addr().unwrap()),
                        );
                        return;
                    }
                    _ => {
                        self.loggers
                            .lock()
                            .unwrap()
                            .log(LogLevel::ERROR, &e.to_string());
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

        let cmd = v0::request::parse(msg)?;

        match cmd {
            Command::CreateDatabase { name } => self.exec_create_db(name),
            Command::OpenDatabase { name } => self.exec_open_db(name),
            Command::CreateTable { name, .. } => self.exec_create_table(name),
            Command::ListDatabases => self.exec_list_databases(),
            Command::ListTables => self.exec_list_tables(),
        }
    }

    fn exec_create_db(&mut self, name: String) -> Result<(), String> {
        if self
            .databases
            .write()
            .unwrap()
            .iter()
            .find(|db| db.name == name)
            .is_some()
        {
            self.stream
                .write(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x00])
                .unwrap();

            return Err(format!(
                "Failed to create database. Name [{}] already in use",
                name
            ));
        }

        let new_db = Database::new(name.clone());

        self.databases.write().unwrap().push(new_db);

        self.loggers
            .lock()
            .unwrap()
            .log(LogLevel::INFO, &format!("Created database [{}]", name));

        self.stream
            .write(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x01])
            .unwrap();

        return Ok(());
    }

    fn exec_open_db(&mut self, name: String) -> Result<(), String> {
        let pos = self
            .databases
            .read()
            .unwrap()
            .iter()
            .position(|db| db.name == name);

        if pos.is_none() {
            return Err(format!(
                "Failed to open database. No database with name [{}]",
                name
            ));
        }

        self.open_db = pos;

        self.loggers
            .lock()
            .unwrap()
            .log(LogLevel::DEBUG, &format!("Opened database [{}]", name));

        return Ok(());
    }

    fn exec_create_table(&mut self, name: String) -> Result<(), String> {
        if self.open_db.is_none() {
            return Err(format!("CREATE TABLE failed. No open database"));
        }

        let open_db_idx = self.open_db.unwrap();

        {
            let open_db = &mut self.databases.write().unwrap()[open_db_idx];

            let tables = &mut open_db.tables;

            if tables.iter().find(|tb| tb.name == name).is_some() {
                return Err(format!(
                    "CREATE TABLE failed. Name [{}::{}] already in use",
                    open_db.name, name
                ));
            }

            tables.push(Table::new(name.clone()));
        }

        // Shadow old mut ref to open_db with a regular ref to open_db
        let open_db = &self.databases.read().unwrap()[open_db_idx];

        self.loggers.lock().unwrap().log(
            LogLevel::INFO,
            &format!("Created table [{}] in database [{}]", name, open_db.name),
        );

        return Ok(());
    }

    fn exec_list_databases(&mut self) -> Result<(), String> {
        let mut output = vec![];

        // TODO: call the v0 resposne serialise function
        output.push(0x03);

        output.extend_from_slice(&(self.databases.read().unwrap().len() as u32).to_le_bytes());

        for db in self.databases.read().unwrap().iter() {
            utils::serialise_string(&db.name, &mut output);
        }

        self.stream
            .write((output.len() as u32).to_le_bytes().as_slice())
            .unwrap();

        self.stream.write(output.as_slice()).unwrap();

        return Ok(());
    }

    fn exec_list_tables(&self) -> Result<(), String> {
        todo!()
    }
}
