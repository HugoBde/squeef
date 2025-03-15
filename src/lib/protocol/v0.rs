use crate::column::Column;
use crate::utils;

#[derive(Debug)]
#[repr(u8)]
pub enum Command {
    CreateDatabase { name: String } = 0x00,
    OpenDatabase { name: String } = 0x01,
    CreateTable { name: String, cols: Vec<Column> } = 0x02,
    Dump = 0xff,
}

type TryFromCommandError = String;

impl TryFrom<&[u8]> for Command {
    type Error = TryFromCommandError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes[0] {
            0x00 => return Self::parse_create_db(&bytes[1..]),
            0x01 => return Self::parse_open_db(&bytes[1..]),
            0x02 => return Self::parse_create_table(&bytes[1..]),
            0xff => return Self::parse_dump(bytes),
            _ => {
                return Err(format!(
                    "Invalid v0 command type. Got type [{:x}]",
                    bytes[1]
                ))
            }
        }
    }
}

impl From<&Command> for u8 {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::CreateDatabase { .. } => 0x00,
            Command::OpenDatabase { .. } => 0x01,
            Command::CreateTable { .. } => 0x02,
            Command::Dump => 0xff,
        }
    }
}

impl From<&Command> for Vec<u8> {
    fn from(cmd: &Command) -> Self {
        let mut bytes = vec![];

        bytes.push(u8::from(cmd));

        match cmd {
            Command::CreateDatabase { name } => cmd.serialise_create_db(name, &mut bytes),
            Command::OpenDatabase { name } => cmd.serialise_open_db(name, &mut bytes),
            Command::CreateTable { name, .. } => cmd.serialise_create_table(name, &mut bytes),
            Command::Dump => {}
        }

        return bytes;
    }
}

impl Command {
    fn parse_create_db(bytes: &[u8]) -> Result<Command, TryFromCommandError> {
        let (bytes, name) = utils::parse_string(bytes)?;

        if bytes.len() > 0 {
            return Err(format!(
                "Remaining data after CREATE_DB command. Got [{:x?}]",
                bytes
            ));
        }

        return Ok(Command::CreateDatabase { name });
    }

    fn parse_open_db(bytes: &[u8]) -> Result<Command, TryFromCommandError> {
        let (bytes, name) = utils::parse_string(bytes)?;

        if bytes.len() > 0 {
            return Err(format!(
                "Remaining data after OPEN_DB command. Got [{:x?}]",
                bytes
            ));
        }

        return Ok(Command::OpenDatabase { name });
    }

    fn parse_create_table(bytes: &[u8]) -> Result<Command, TryFromCommandError> {
        let (bytes, name) = utils::parse_string(bytes)?;

        if bytes.len() > 0 {
            return Err(format!(
                "Remaining data after CREATE_TABLE command. Got [{:x?}]",
                bytes
            ));
        }

        return Ok(Command::CreateTable { name, cols: vec![] });
    }

    fn parse_dump(_: &[u8]) -> Result<Command, TryFromCommandError> {
        return Ok(Command::Dump);
    }

    fn serialise_create_db(&self, name: &String, bytes: &mut Vec<u8>) {
        utils::serialise_string(name, bytes);
    }

    fn serialise_open_db(&self, name: &String, bytes: &mut Vec<u8>) {
        utils::serialise_string(name, bytes);
    }

    fn serialise_create_table(&self, name: &String, bytes: &mut Vec<u8>) {
        utils::serialise_string(name, bytes);
    }
}
