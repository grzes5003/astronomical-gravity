use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::net::AddrParseError;

trait New<T>: Sized {
    fn new(_: T) -> Self;
}

#[derive(Debug)]
pub struct StarErr {
    msg: String
}

impl From<&str> for StarErr {
    fn from(msg: &str) -> Self {
        StarErr {
            msg: msg.to_string()
        }
    }
}

impl From<String> for StarErr {
    fn from(msg: String) -> Self {
        StarErr {
            msg
        }
    }
}


impl Display for StarErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: {}", self.msg)
    }
}

impl Error for StarErr {}