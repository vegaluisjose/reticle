use asm::errors::Error as AsmError;
use ir::errors::Error as IrError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Ir(IrError),
    Asm(AsmError),
    ISel(String),
}

impl Error {
    pub fn new_isel_error(msg: &str) -> Self {
        Error::ISel(msg.to_string())
    }
}

impl From<IrError> for Error {
    fn from(e: IrError) -> Self {
        Error::Ir(e)
    }
}

impl From<AsmError> for Error {
    fn from(e: AsmError) -> Self {
        Error::Asm(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ir(msg) => write!(f, "{}", msg),
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::ISel(msg) => write!(f, "{}", msg),
        }
    }
}
