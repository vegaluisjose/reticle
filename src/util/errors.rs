use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "[Error][Parsing] {}", msg),
            Error::ConvError(msg) => write!(f, "[Error][Conversion] {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    ConvError(String),
}

impl Error {
    pub fn new_parse_error(msg: &str) -> Self {
        Error::ParseError(msg.to_string())
    }
    pub fn new_conv_error(msg: &str) -> Self {
        Error::ConvError(msg.to_string())
    }
}
