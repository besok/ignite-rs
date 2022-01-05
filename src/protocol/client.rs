use std::net::TcpStream;
use super::{handshake_request,handshake_response, error::HandshakeError, config::IgniteClientCfg};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{io::{self, Read, Write}};

pub struct IgniteClient{
    stream:TcpStream,
    cfg:IgniteClientCfg,
}

impl IgniteClient {
    fn connect(cfg:IgniteClientCfg) -> Result<Self,HandshakeError>{
        let request = handshake_request(cfg.username.clone(),cfg.password.clone(),cfg.major,cfg.minor,cfg.patch)?;
        let mut stream = TcpStream::connect(cfg.nodes.get(0).unwrap().address())?;
        stream.write_all(request.as_slice())?;
        handshake_response(&mut stream)?;
        Ok(IgniteClient{stream,cfg})
    }
}


#[cfg(test)]
mod tests {
    use crate::protocol::client::{IgniteClient, IgniteClientCfg};

   
    #[test]
    fn handshake_test() {
        assert!(IgniteClient::connect(IgniteClientCfg::default(1,6)).is_ok());
        assert!(IgniteClient::connect(IgniteClientCfg::default(4, 2)).is_err());
    }
}