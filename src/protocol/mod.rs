mod error;
mod client;
mod config;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{io::{self, Read, Write}};
use self::error::HandshakeError;

pub fn handshake_request(
    username: String,
    password: String,
    major: u16,
    minor: u16,
    patch: u16,
) -> io::Result<Vec<u8>> {
    let mut buf = vec![];
    buf.write_i32::<LittleEndian>(8 + username.len() as i32 + password.len() as i32)?;
    buf.write_u8(1)?;
    buf.write_u16::<LittleEndian>(major)?;
    buf.write_u16::<LittleEndian>(minor)?;
    buf.write_u16::<LittleEndian>(patch)?;
    buf.write_u8(2)?;
    buf.write(&username.into_bytes())?;
    buf.write(&password.into_bytes())?;

    Ok(buf)
}

pub fn handshake_response(reader: &mut impl Read) -> Result<(),HandshakeError> {
    let len = reader.read_i32::<LittleEndian>()? as usize;
    let flag = reader.read_u8()?;
    match flag {
        1 => Ok(()),
        _ => {
            let v_major = reader.read_u16::<LittleEndian>()?; 
            let v_minor = reader.read_u16::<LittleEndian>()?; 
            let v_patch = reader.read_u16::<LittleEndian>()?; 
            
            let mut bufferString = vec![0 as u8; len-8];
            reader.read_exact(bufferString.as_mut_slice())?;
            let mes = String::from_utf8(bufferString)?;
            Err(HandshakeError{ v_major, v_minor, v_patch, mes: mes.to_string() })
        }
    
    }


}

#[cfg(test)]
mod tests {
    use std::{net::TcpStream, vec, io::Write};

    use super::{handshake_response,handshake_request};

    #[test]
    fn req_test() {
        let res =
        handshake_request("username".to_string(), "password".to_string(), 1, 3, 2).unwrap();
        assert_eq!(
            res,
            vec![
                24, 0, 0, 0, 1, 1, 0, 3, 0, 2, 0, 2, 117, 115, 101, 114, 110, 97, 109, 101, 112,
                97, 115, 115, 119, 111, 114, 100
            ]
        );
    }

    #[test]
    fn res_test() {
        let mut stream = TcpStream::connect("127.0.0.1:10800").unwrap();
        let req =  handshake_request("".to_string(),"".to_string(), 1, 6, 0).unwrap();

        stream.write_all(req.as_slice()).unwrap();
        
        handshake_response(&mut stream).unwrap();
    }
    
}
