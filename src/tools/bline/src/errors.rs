use ir::errors::Error as IRError;
use std::fmt;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    IR(IRError),
    Bline(String),
    TryFromInt(TryFromIntError),
}

impl Error {
    pub fn new_bline_error(msg: &str) -> Self {
        Error::Bline(msg.to_string())
    }
}

impl From<IRError> for Error {
    fn from(e: IRError) -> Self {
        Error::IR(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IR(msg) => write!(f, "{}", msg),
            Error::Bline(msg) => write!(f, "{}", msg),
            Error::TryFromInt(msg) => write!(f, "{}", msg),
        }
    }
}
