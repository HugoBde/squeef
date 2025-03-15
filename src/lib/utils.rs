use std::io::{self, Read};
use std::mem;
use std::net::TcpStream;

pub fn read_msg(reader: &mut TcpStream) -> Result<Vec<u8>, io::Error> {
    let mut buf: [u8; 4] = [0; 4];

    reader.read_exact(&mut buf)?;

    let read_len = u32::from_le_bytes(buf) as usize;

    let mut output = vec![0; read_len];

    reader.read_exact(output.as_mut_slice())?;

    return Ok(output);
}

pub fn parse_string(bytes: &[u8]) -> Result<(&[u8], String), String> {
    if bytes.len() < mem::size_of::<u32>() {
        return Err(format!(
            "Data too short to hold string length. Got data length {}",
            bytes.len()
        ));
    }

    let len = u32::from_le_bytes(bytes[0..mem::size_of::<u32>()].try_into().unwrap()) as usize;

    let bytes = &bytes[mem::size_of::<u32>()..];

    if len == 0 {
        return Ok((bytes, String::new()));
    }

    let string = String::from_utf8(Vec::from(&bytes[..len]))
        .map_err(|e| format!("Invalid UTF-8 string. Got bytes {:x?}", e.into_bytes()))?;

    let bytes = &bytes[len..];

    return Ok((bytes, string));
}

pub fn serialise_string(string: &String, bytes: &mut Vec<u8>) {
    let len = string.len() as u32;
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(string.as_bytes());
}
