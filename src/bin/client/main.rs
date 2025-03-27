// Std Lib Imports
use std::io::{stdin, stdout, Write};
use std::net;

// Executable Imports
mod config;
mod lang;

// Squeef Lib Imports
use squeef::protocol::v0;

// Third Party Imports
use clap::Parser;
use squeef::utils;

const BANNER: &str = r#"
 _____ _____ _   _ _____ ___________     _____ _   _ _____ _     _     
/  ___|  _  | | | |  ___|  ___|  ___|   /  ___| | | |  ___| |   | |    
\ `--.| | | | | | | |__ | |__ | |_ _____\ `--.| |_| | |__ | |   | |    
 `--. | | | | | | |  __||  __||  _|______`--. |  _  |  __|| |   | |    
/\__/ \ \/' | |_| | |___| |___| |       /\__/ | | | | |___| |___| |____
\____/ \_/\_\\___/\____/\____/\_|       \____/\_| |_\____/\_____\_____/

By @ssidbde

"#;

fn main() -> () {
    println!("{}", BANNER);

    let config = config::Config::parse();

    println!("Connecting to {}:{} ...", config.host, config.port);

    match net::TcpStream::connect((config.host, config.port)) {
        Ok(stream) => {
            println!("Connected!");
            run(stream);
        }

        Err(e) => {
            eprintln!("{}", e);
        }
    }

    println!("\nBye!");
}

fn run(mut stream: net::TcpStream) -> () {
    print!("> ");
    stdout().flush().unwrap();

    for line in stdin().lines() {
        let line = line.unwrap();
        match lang::parse(line) {
            Ok(cmd) => {
                let data: Vec<u8> = v0::request::serialise(cmd);
                let data_len = data.len() as u32;
                stream.write(&data_len.to_le_bytes()).unwrap();
                stream.write(data.as_slice()).unwrap();
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        let data = utils::read_msg(&mut stream).unwrap();

        match v0::response::parse(&data) {
            Ok(resp) => {
                println!("{:?}", resp);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
        print!("> ");
        stdout().flush().unwrap();
    }
}
