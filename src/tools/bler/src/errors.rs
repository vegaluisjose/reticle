use asm::errors::Error as AsmError;
use ir::errors::Error as IRError;
use std::fmt;
use xir::errors::Error as XirError;

#[derive(Debug)]
pub enum Error {
    IR(IRError),
    Asm(AsmError),
    Xir(XirError),
    Bler(String),
}

impl Error {
    pub fn new_bler_error(msg: &str) -> Self {
        Error::Bler(msg.to_string())
    }
}

impl From<AsmError> for Error {
    fn from(e: AsmError) -> Self {
        Error::Asm(e)
    }
}

impl From<XirError> for Error {
    fn from(e: XirError) -> Self {
        Error::Xir(e)
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
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::Xir(msg) => write!(f, "{}", msg),
            Error::Bler(msg) => write!(f, "{}", msg),
        }
    }
}
