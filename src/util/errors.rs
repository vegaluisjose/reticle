use crate::xl::parser as xlparser;
use std::fmt;
use std::num::TryFromIntError;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::XLParse(msg) => write!(f, "[Error][Parsing] {}", msg),
            Error::Conversion(msg) => write!(f, "[Error][Conversion] {}", msg),
            Error::Type(msg) => write!(f, "[Error][Type] {}", msg),
            Error::TryFromInt(msg) => write!(f, "[Error][TryFromInt] {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    XLParse(pest_consume::Error<xlparser::Rule>),
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

impl From<pest_consume::Error<xlparser::Rule>> for Error {
    fn from(e: pest_consume::Error<xlparser::Rule>) -> Self {
        Error::XLParse(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}
