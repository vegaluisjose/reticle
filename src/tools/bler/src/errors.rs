use asm::errors::Error as AsmError;
use ir::errors::Error as IrError;
use std::fmt;
use xir::errors::Error as XirError;

#[derive(Debug)]
pub enum Error {
    Ir(IrError),
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
            Error::Xir(msg) => write!(f, "{}", msg),
            Error::Bler(msg) => write!(f, "{}", msg),
        }
    }
}
