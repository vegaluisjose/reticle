use asm::errors::Error as AsmError;
use ir::errors::Error as IRError;
use isel::errors::Error as ISelError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Opt(String),
    Driver(String),
    IR(IRError),
    Asm(AsmError),
    ISel(ISelError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IR(msg) => write!(f, "{}", msg),
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::ISel(msg) => write!(f, "{}", msg),
            Error::Opt(msg) => write!(f, "{}", msg),
            Error::Driver(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error {
    pub fn new_opt_error(msg: &str) -> Self {
        Error::Opt(msg.to_string())
    }
    pub fn new_driver_error(msg: &str) -> Self {
        Error::Driver(msg.to_string())
    }
}

impl From<IRError> for Error {
    fn from(e: IRError) -> Self {
        Error::IR(e)
    }
}

impl From<AsmError> for Error {
    fn from(e: AsmError) -> Self {
        Error::Asm(e)
    }
}

impl From<ISelError> for Error {
    fn from(e: ISelError) -> Self {
        Error::ISel(e)
    }
}
