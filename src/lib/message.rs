use std::io::{self, BufRead, BufReader, Read};
use std::net::TcpStream;

pub fn read_msg(
    reader: &mut BufReader<TcpStream>,
) -> Result<Vec<u8>, io::Error> {
    let mut buf: [u8; 4] = [0; 4];

    reader.read_exact(&mut buf)?;

    let mut read_len = u32::from_le_bytes(buf) as usize;

    let mut output = Vec::with_capacity(read_len);

    loop {
        let bytes = reader.fill_buf()?;

        if bytes.len() > read_len {
            output.extend_from_slice(&bytes[0..read_len]);
            reader.consume(read_len);
            return Ok(output);
        } else {
            let len = bytes.len();
            read_len -= len;
            output.extend_from_slice(bytes);
            reader.consume(len);
        }
    }
}
