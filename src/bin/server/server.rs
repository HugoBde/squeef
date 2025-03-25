use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};

use squeef::database::Database;
use squeef::protocol::v0::{self, Command};
use squeef::table::Table;
use squeef::utils;

use crate::log::{LogLevel, Loggers};
use crate::thread_pool::ThreadPool;

#[derive(Debug)]
pub struct Server {
    port: u16,
    thread_pool: ThreadPool,

    databases: Arc<RwLock<Vec<Database>>>,

    loggers: Arc<Mutex<Loggers>>,
}

impl Server {
    pub fn new(port: u16, max_concurrent_connection: isize, loggers: Loggers) -> Server {
        Server {
            port,
            loggers: Arc::new(Mutex::new(loggers)),
            thread_pool: ThreadPool::new(max_concurrent_connection),
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
                    let mut client_connection =
                        ClientConnection::new(stream, self.databases.clone(), self.loggers.clone());
                    match self.thread_pool.spawn(move || client_connection.run()) {
                        Ok(_) => (),
                        Err(e) => {
                            self.loggers
                                .lock()
                                .unwrap()
                                .log(LogLevel::ERROR, &e.to_string());
                            return;
                        }
                    }
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

struct ClientConnection {
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
                        self.loggers
                            .lock()
                            .unwrap()
                            .log(LogLevel::INFO, "Connection closed");
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

        let cmd = v0::Command::try_from(msg)?;

        match cmd {
            Command::CreateDatabase { name } => self.exec_create_db(name),
            Command::OpenDatabase { name } => self.exec_open_db(name),
            Command::CreateTable { name, .. } => self.exec_create_table(name),
            Command::Dump => self.exec_dump(),
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

    fn exec_dump(&self) -> Result<(), String> {
        if self.open_db.is_none() {
            return Err(format!("DUMP failed. No open database"));
        }

        let open_db = &self.databases.read().unwrap()[self.open_db.unwrap()];

        eprintln!("DUMP: {open_db:#?}");

        return Ok(());
    }
}
