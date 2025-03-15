use std::io::Write;
use std::net;

use squeef::protocol::v0;

fn main() -> () {
    let mut stream = net::TcpStream::connect(("127.0.0.1", 6870)).unwrap();
    let cmds = vec![
        v0::Command::CreateDatabase {
            name: String::from("my_db"),
        },
        v0::Command::OpenDatabase {
            name: String::from("my_db"),
        },
        v0::Command::CreateTable {
            name: String::from("my_table"),
            cols: vec![],
        },
        v0::Command::Dump,
    ];
    for cmd in cmds {
        let data: Vec<u8> = cmd.into();
        stream.write(data.as_slice()).unwrap();
    }
}
