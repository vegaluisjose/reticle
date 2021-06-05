use ir::errors::Error as IrError;
use std::fmt;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    Ir(IrError),
    Bline(String),
    TryFromInt(TryFromIntError),
}

impl Error {
    pub fn new_bline_error(msg: &str) -> Self {
        Error::Bline(msg.to_string())
    }
}

impl From<IrError> for Error {
    fn from(e: IrError) -> Self {
        Error::Ir(e)
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
            Error::Ir(msg) => write!(f, "{}", msg),
            Error::Bline(msg) => write!(f, "{}", msg),
            Error::TryFromInt(msg) => write!(f, "{}", msg),
        }
    }
}
