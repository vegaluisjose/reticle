use ir::errors::Error as IRError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IR(IRError),
    BLine(String),
}

impl Error {
    pub fn new_bline_error(msg: &str) -> Self {
        Error::BLine(msg.to_string())
    }
}

impl From<IRError> for Error {
    fn from(e: IRError) -> Self {
        Error::IR(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IR(msg) => write!(f, "{}", msg),
            Error::BLine(msg) => write!(f, "{}", msg),
        }
    }
}
