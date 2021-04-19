use crate::parser;
use std::fmt;
use std::num::ParseIntError;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    Parser(pest_consume::Error<parser::Rule>),
    Conversion(String),
    Type(String),
    TryFromInt(TryFromIntError),
    ParseInt(ParseIntError),
}

impl Error {
    pub fn new_conv_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
    pub fn new_type_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
}

impl From<pest_consume::Error<parser::Rule>> for Error {
    fn from(e: pest_consume::Error<parser::Rule>) -> Self {
        Error::Parser(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser(msg) => write!(f, "{}", msg),
            Error::ParseInt(msg) => write!(f, "{}", msg),
            Error::Conversion(msg) => write!(f, "{}", msg),
            Error::Type(msg) => write!(f, "{}", msg),
            Error::TryFromInt(msg) => write!(f, "{}", msg),
        }
    }
}
