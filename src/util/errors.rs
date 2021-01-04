use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "[Error][Parsing] {}", msg),
            Error::Conversion(msg) => write!(f, "[Error][Conversion] {}", msg),
            Error::Type(msg) => write!(f, "[Error][Type] {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Conversion(String),
    Type(String),
}

impl Error {
    pub fn new_parse_error(msg: &str) -> Self {
        Error::Parse(msg.to_string())
    }
    pub fn new_conv_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
    pub fn new_type_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
}
