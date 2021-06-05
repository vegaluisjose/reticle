use asm::errors::Error as AsmError;
use ir::errors::Error as IrError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Ir(IrError),
    Asm(AsmError),
    Driver(String),
    Opt(String),
}

impl Error {
    pub fn new_opt_error(msg: &str) -> Self {
        Error::Opt(msg.to_string())
    }
    pub fn new_driver_error(msg: &str) -> Self {
        Error::Driver(msg.to_string())
    }
}

impl From<AsmError> for Error {
    fn from(e: AsmError) -> Self {
        Error::Asm(e)
    }
}

impl From<IrError> for Error {
    fn from(e: IrError) -> Self {
        Error::Ir(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ir(msg) => write!(f, "{}", msg),
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::Driver(msg) => write!(f, "{}", msg),
            Error::Opt(msg) => write!(f, "{}", msg),
        }
    }
}
