use asm::errors::Error as AsmError;
use bler::errors::Error as BlerError;
use bline::errors::Error as BlineError;
use ir::errors::Error as IrError;
use isel::errors::Error as ISelError;
use std::fmt;
use xir::errors::Error as XirError;
use xpand::errors::Error as XpandError;

#[derive(Debug)]
pub enum Error {
    Opt(String),
    Driver(String),
    Ir(IrError),
    Asm(AsmError),
    Xir(XirError),
    ISel(ISelError),
    Bler(BlerError),
    Bline(BlineError),
    Xpand(XpandError),
}

impl Error {
    pub fn new_opt_error(msg: &str) -> Self {
        Error::Opt(msg.to_string())
    }
    pub fn new_driver_error(msg: &str) -> Self {
        Error::Driver(msg.to_string())
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

impl From<XirError> for Error {
    fn from(e: XirError) -> Self {
        Error::Xir(e)
    }
}

impl From<ISelError> for Error {
    fn from(e: ISelError) -> Self {
        Error::ISel(e)
    }
}

impl From<BlerError> for Error {
    fn from(e: BlerError) -> Self {
        Error::Bler(e)
    }
}

impl From<BlineError> for Error {
    fn from(e: BlineError) -> Self {
        Error::Bline(e)
    }
}

impl From<XpandError> for Error {
    fn from(e: XpandError) -> Self {
        Error::Xpand(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ir(msg) => write!(f, "{}", msg),
            Error::Asm(msg) => write!(f, "{}", msg),
            Error::Xir(msg) => write!(f, "{}", msg),
            Error::ISel(msg) => write!(f, "{}", msg),
            Error::Bler(msg) => write!(f, "{}", msg),
            Error::Bline(msg) => write!(f, "{}", msg),
            Error::Opt(msg) => write!(f, "{}", msg),
            Error::Driver(msg) => write!(f, "{}", msg),
            Error::Xpand(msg) => write!(f, "{}", msg),
        }
    }
}
