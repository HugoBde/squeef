use crate::command::Command;
use crate::utils;

#[repr(u8)]
enum CommandDiscriminant {
    CreateDatabase = 0x00,
    OpenDatabase = 0x01,
    CreateTable = 0x02,
    ListDatabases = 0x03,
    ListTables = 0x04,
}

impl From<u8> for CommandDiscriminant {
    fn from(byte: u8) -> Self {
        return match byte {
            0x00 => CommandDiscriminant::CreateDatabase,
            0x01 => CommandDiscriminant::OpenDatabase,
            0x02 => CommandDiscriminant::CreateTable,
            0x03 => CommandDiscriminant::ListDatabases,
            0x04 => CommandDiscriminant::ListTables,
            _ => panic!("Unknown command discriminant [{:x}]", byte),
        };
    }
}

impl From<CommandDiscriminant> for u8 {
    fn from(cmd: CommandDiscriminant) -> Self {
        return match cmd {
            CommandDiscriminant::CreateDatabase => 0x00,
            CommandDiscriminant::OpenDatabase => 0x01,
            CommandDiscriminant::CreateTable => 0x02,
            CommandDiscriminant::ListDatabases => 0x03,
            CommandDiscriminant::ListTables => 0x04,
        };
    }
}

pub mod request {
    use super::utils;
    use super::Command;
    use super::CommandDiscriminant;

    pub fn parse(bytes: &[u8]) -> Result<Command, String> {
        let cmd = CommandDiscriminant::from(bytes[0]);

        return match cmd {
            CommandDiscriminant::CreateDatabase => parse_create_db(&bytes[1..]),
            CommandDiscriminant::OpenDatabase => parse_open_db(&bytes[1..]),
            CommandDiscriminant::CreateTable => parse_create_table(&bytes[1..]),
            CommandDiscriminant::ListDatabases => Ok(Command::ListDatabases),
            CommandDiscriminant::ListTables => Ok(Command::ListTables),
        };
    }

    pub fn serialise(cmd: Command) -> Vec<u8> {
        let mut bytes = vec![];

        match cmd {
            Command::CreateDatabase { name } => serialise_create_db(name, &mut bytes),
            Command::OpenDatabase { name } => serialise_open_db(name, &mut bytes),
            Command::CreateTable { name, .. } => serialise_create_table(name, &mut bytes),
            Command::ListDatabases => serialise_list_databases(&mut bytes),
            Command::ListTables => serialise_list_tables(&mut bytes),
        }

        return bytes;
    }

    fn parse_create_db(bytes: &[u8]) -> Result<Command, String> {
        let (bytes, name) = utils::parse_string(bytes)?;

        if bytes.len() > 0 {
            return Err(format!(
                "Remaining data after CREATE_DB command. Got [{:x?}]",
                bytes
            ));
        }

        return Ok(Command::CreateDatabase { name });
    }

    fn parse_open_db(bytes: &[u8]) -> Result<Command, String> {
        let (bytes, name) = utils::parse_string(bytes)?;

        if bytes.len() > 0 {
            return Err(format!(
                "Remaining data after OPEN DATABASE command. Got [{:x?}]",
                bytes
            ));
        }

        return Ok(Command::OpenDatabase { name });
    }

    fn parse_create_table(bytes: &[u8]) -> Result<Command, String> {
        let (bytes, name) = utils::parse_string(bytes)?;

        if bytes.len() > 0 {
            return Err(format!(
                "Remaining data after CREATE_TABLE command. Got [{:x?}]",
                bytes
            ));
        }

        return Ok(Command::CreateTable { name, cols: vec![] });
    }

    fn serialise_create_db(name: String, bytes: &mut Vec<u8>) {
        bytes.push(CommandDiscriminant::CreateDatabase.into());
        utils::serialise_string(&name, bytes);
    }

    fn serialise_open_db(name: String, bytes: &mut Vec<u8>) {
        bytes.push(CommandDiscriminant::OpenDatabase.into());
        utils::serialise_string(&name, bytes);
    }

    fn serialise_create_table(name: String, bytes: &mut Vec<u8>) {
        bytes.push(CommandDiscriminant::CreateTable.into());
        utils::serialise_string(&name, bytes);
    }

    fn serialise_list_databases(bytes: &mut Vec<u8>) {
        bytes.push(CommandDiscriminant::ListDatabases.into());
    }

    fn serialise_list_tables(bytes: &mut Vec<u8>) {
        bytes.push(CommandDiscriminant::ListTables.into());
    }
}

pub mod response {

    use super::utils;
    use super::CommandDiscriminant;

    pub fn parse(bytes: &[u8]) -> Result<String, String> {
        let cmd = CommandDiscriminant::from(bytes[0]);

        return match cmd {
            CommandDiscriminant::CreateDatabase => parse_create_db(&bytes[1..]),
            CommandDiscriminant::OpenDatabase => parse_open_db(&bytes[1..]),
            CommandDiscriminant::CreateTable => parse_create_table(&bytes[1..]),
            CommandDiscriminant::ListDatabases => parse_list_databases(&bytes[1..]),
            CommandDiscriminant::ListTables => parse_list_tables(&bytes[1..]),
        };
    }

    pub fn serialise() {
        todo!()
    }

    fn parse_create_db(bytes: &[u8]) -> Result<String, String> {
        let (_, success) = utils::parse_bool(bytes)?;
        match success {
            true => return Ok(String::from("Database created successfully")),
            false => return Err(String::from("Failed to create database")),
        }
    }

    fn parse_open_db(_bytes: &[u8]) -> Result<String, String> {
        return Ok(String::from("TODO: RESP OPEN_DB"));
    }

    fn parse_create_table(_bytes: &[u8]) -> Result<String, String> {
        let (_, success) = utils::parse_bool(_bytes)?;
        match success {
            true => return Ok(String::from("Table created successfully")),
            false => return Err(String::from("Failed to create table")),
        }
    }

    fn parse_list_databases(bytes: &[u8]) -> Result<String, String> {
        let (mut bytes, db_count) = utils::parse_u32(bytes)?;

        let mut res = String::from("Databases: [");

        for _ in 0..db_count {
            let (new_bytes, db_name) = utils::parse_string(bytes)?;
            bytes = new_bytes;
            res.push_str(&db_name);
            res.push(',');
        }

        res.pop();
        res.push(']');

        return Ok(res);
    }

    fn parse_list_tables(_bytes: &[u8]) -> Result<String, String> {
        return Ok(String::from("TODO: RESP LIST_TABLES"));
    }
}
