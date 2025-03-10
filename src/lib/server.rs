use std::io::{BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream};

use crate::database::Database;
use crate::log::Logger;
use crate::message;
use crate::table::Table;

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
            match message::read_msg(&mut reader) {
                Ok(msg) => match self.process_msg(msg.as_slice()) {
                    Err(e) => self.log_error(e.to_string()),
                    _ => {}
                },
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

        match msg[0] {
            0x00 => return self.process_msg_v0(&msg[1..]),
            _ => {
                return Err(format!(
                    "Invalid message version. Got version {:x}",
                    msg[0]
                ))
            }
        }
    }

    fn process_msg_v0(&mut self, msg: &[u8]) -> Result<(), String> {
        match msg[0] {
            0x00 => return self.process_msg_v0_create_db(&msg[1..]),
            0x01 => return self.process_msg_v0_open_db(&msg[1..]),
            0x02 => return self.process_msg_v0_create_table(&msg[1..]),
            0xff => return self.process_msg_v0_dump(msg),
            _ => {
                return Err(format!(
                    "Invalid command. Got command {:x}",
                    msg[1]
                ))
            }
        }
    }

    fn process_msg_v0_create_db(&mut self, msg: &[u8]) -> Result<(), String> {
        if msg.len() < 4 {
            return Err(format!(
                "Invalid CREATE_DB command. Missing Database name header"
            ));
        }

        let db_name_len =
            u32::from_le_bytes(msg[0..4].try_into().unwrap()) as usize;

        let msg = &msg[4..];

        if msg.len() < db_name_len {
            return Err(format!(
                "Invalid CREATE_DB command. Message too short for Database name. Expected {} bytes, got {} bytes", db_name_len, msg.len()
            ));
        }

        let name = String::from_utf8(Vec::from(msg));

        if name.is_err() {
            return Err(format!(
                "Invalid CREATE_DB command. Failed to decode Database name as UTF-8 string. Got {:x?}", msg
            ));
        }

        let name = name.unwrap();

        if let Some(_) = self.databases.iter().find(|db| db.name == name) {
            return Err(format!(
                "CREATE_DB error. Database {} already exists",
                name
            ));
        }

        let new_db = Database::new(name.clone());

        self.log_info(format!("Created Database {}", name));

        self.databases.push(new_db);
        return Ok(());
    }

    fn process_msg_v0_open_db(&mut self, msg: &[u8]) -> Result<(), String> {
        if msg.len() < 4 {
            return Err(format!(
                "Invalid OPEN_DB command. Missing Database name header"
            ));
        }

        let db_name_len =
            u32::from_le_bytes(msg[0..4].try_into().unwrap()) as usize;

        let msg = &msg[4..];

        if msg.len() < db_name_len {
            return Err(format!(
                "Invalid OPEN_DB command. Message too short for Database name. Expected {} bytes, got {} bytes", db_name_len, msg.len()
            ));
        }

        let name = String::from_utf8(Vec::from(msg));

        if name.is_err() {
            return Err(format!(
                "Invalid OPEN_DB command. Failed to decode Database name as UTF-8 string. Got {:x?}", msg
            ));
        }

        let name = name.unwrap();

        match self.databases.iter().position(|db| db.name == name) {
            Some(idx) => self.open_db = Some(idx),
            None => {
                return Err(format!(
                    "OPEN_DB error. Database {} doesn't exist",
                    name
                ))
            }
        }

        return Ok(());
    }

    fn process_msg_v0_create_table(
        &mut self,
        msg: &[u8],
    ) -> Result<(), String> {
        if self.open_db.is_none() {
            return Err(String::from("CREATE_TABLE error. No Database open"));
        }

        let open_db = self.open_db.unwrap();

        if msg.len() < 4 {
            return Err(format!(
                "Invalid CREATE_TABLE command. Missing Table name header"
            ));
        }

        let table_name_len =
            u32::from_le_bytes(msg[0..4].try_into().unwrap()) as usize;

        let msg = &msg[4..];

        if msg.len() < table_name_len {
            return Err(format!(
                "Invalid CREATE_TABLE command. Message too short for Table name. Expected {} bytes, got {} bytes", table_name_len, msg.len()
            ));
        }

        let name = String::from_utf8(Vec::from(msg));

        if name.is_err() {
            return Err(format!(
                "Invalid CREATE_TABLE command. Failed to decode Table name as UTF-8 string. Got {:?}", msg
            ));
        }

        let table_name = name.unwrap();

        if let Some(_) = self.databases[open_db].tables.get(&table_name) {
            return Err(format!(
                "CREATE_TABLE error. Table {} already exists in Database {}",
                table_name, self.databases[open_db].name
            ));
        }

        let table = Table::new(table_name.clone());

        self.log_info(format!(
            "Created Table {} in Database {}",
            table.name, self.databases[open_db].name
        ));

        self.databases[open_db].tables.insert(table_name, table);

        return Ok(());
    }

    fn process_msg_v0_dump(&mut self, _: &[u8]) -> Result<(), String> {
        eprintln!("{self:?}");
        return Ok(());
    }

    fn log_info(&mut self, msg: String) {
        self.loggers.iter_mut().for_each(|l| l.info(&msg).unwrap());
    }

    fn log_error(&mut self, msg: String) {
        self.loggers.iter_mut().for_each(|l| l.error(&msg).unwrap());
    }
}
