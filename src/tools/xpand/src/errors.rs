use ir::errors::Error as IRError;
use std::fmt;
use std::num::TryFromIntError;
use xir::errors::Error as XirError;

#[derive(Debug)]
pub enum Error {
    IR(IRError),
    Xir(XirError),
    Xpand(String),
    TryFromInt(TryFromIntError),
}

impl Error {
    pub fn new_xpand_error(msg: &str) -> Self {
        Error::Xpand(msg.to_string())
    }
}

impl From<IRError> for Error {
    fn from(e: IRError) -> Self {
        Error::IR(e)
    }
}

impl From<XirError> for Error {
    fn from(e: XirError) -> Self {
        Error::Xir(e)
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
            Error::Xir(msg) => write!(f, "{}", msg),
            Error::Xpand(msg) => write!(f, "{}", msg),
            Error::TryFromInt(msg) => write!(f, "{}", msg),
        }
    }
}
