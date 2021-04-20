use asm::errors::Error as AsmError;
use xir::errors::Error as XirError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Asm(AsmError),
    Xir(XirError),
    Assembler(String),
}

impl Error {
    pub fn new_assembler_error(msg: &str) -> Self {
        Error::Assembler(msg.to_string())
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::Xir(msg) => write!(f, "{}", msg),
            Error::Assembler(msg) => write!(f, "{}", msg),
        }
    }
}
