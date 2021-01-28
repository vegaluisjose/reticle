use crate::ir::parser as irparser;
use crate::tdl::parser as tdlparser;
use crate::xl::parser as xlparser;
use std::fmt;
use std::num::TryFromIntError;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IRParse(msg) => write!(f, "[Error][IR parsing] {}", msg),
            Error::XLParse(msg) => write!(f, "[Error][XL parsing] {}", msg),
            Error::TDLParse(msg) => write!(f, "[Error][TDL parsing] {}", msg),
            Error::Conversion(msg) => write!(f, "[Error][Conversion] {}", msg),
            Error::Type(msg) => write!(f, "[Error][Type] {}", msg),
            Error::TryFromInt(msg) => write!(f, "[Error][TryFromInt] {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IRParse(pest_consume::Error<irparser::Rule>),
    XLParse(pest_consume::Error<xlparser::Rule>),
    TDLParse(pest_consume::Error<tdlparser::Rule>),
    Conversion(String),
    Type(String),
    TryFromInt(TryFromIntError),
}

impl Error {
    pub fn new_conv_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
    pub fn new_type_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
}

impl From<pest_consume::Error<irparser::Rule>> for Error {
    fn from(e: pest_consume::Error<irparser::Rule>) -> Self {
        Error::IRParse(e)
    }
}

impl From<pest_consume::Error<xlparser::Rule>> for Error {
    fn from(e: pest_consume::Error<xlparser::Rule>) -> Self {
        Error::XLParse(e)
    }
}

impl From<pest_consume::Error<tdlparser::Rule>> for Error {
    fn from(e: pest_consume::Error<tdlparser::Rule>) -> Self {
        Error::TDLParse(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}
