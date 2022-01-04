use std::{string::FromUtf8Error, io};

#[derive(Debug)]
pub struct HandshakeError{
    pub v_major:u16,
    pub v_minor:u16,
    pub v_patch:u16,
    pub mes:String,
}

impl From<io::Error> for HandshakeError{
    fn from(error: io::Error) -> Self {
        format!("{:?}",error);
        HandshakeError{ v_major: 0, v_minor: 0, v_patch: 0, mes:format!("{:?}",error) }
    }
}
impl From<FromUtf8Error> for HandshakeError{
    fn from(error: FromUtf8Error) -> Self {
        format!("{:?}",error);
        HandshakeError{ v_major: 0, v_minor: 0, v_patch: 0, mes:format!("{:?}",error) }
    }
}