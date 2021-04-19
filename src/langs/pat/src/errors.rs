use crate::parser;
use std::fmt;
use std::num::ParseIntError;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    Parse(pest_consume::Error<parser::Rule>),
    Type(String),
    TryFromInt(TryFromIntError),
    ParseInt(ParseIntError),
}

impl From<pest_consume::Error<parser::Rule>> for Error {
    fn from(e: pest_consume::Error<parser::Rule>) -> Self {
        Error::Parse(e)
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
            Error::Parse(msg) => write!(f, "{}", msg),
            Error::ParseInt(msg) => write!(f, "{}", msg),
            Error::Type(msg) => write!(f, "{}", msg),
            Error::TryFromInt(msg) => write!(f, "{}", msg),
        }
    }
}
