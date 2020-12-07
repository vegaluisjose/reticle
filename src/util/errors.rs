use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

impl Error {
    pub fn new_parse_error(msg: &str) -> Self {
        Error::ParseError(msg.to_string())
    }
}
