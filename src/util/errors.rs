use crate::asm::parser as asmparser;
use crate::ir::parser as irparser;
use crate::tdl::parser as tdlparser;
use crate::xl::parser as xlparser;
use std::fmt;
use std::num::ParseIntError;
use std::num::TryFromIntError;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IRParse(msg) => write!(f, "[Error][IR] {}", msg),
            Error::AsmParse(msg) => write!(f, "[Error][ASM] {}", msg),
            Error::XLParse(msg) => write!(f, "[Error][XL] {}", msg),
            Error::TDLParse(msg) => write!(f, "[Error][TDL] {}", msg),
            Error::Conversion(msg) => write!(f, "[Error][Conversion] {}", msg),
            Error::Type(msg) => write!(f, "[Error][Type] {}", msg),
            Error::ParseInt(msg) => write!(f, "[Error][ParseInt] {}", msg),
            Error::TryFromInt(msg) => write!(f, "[Error][TryFromInt] {}", msg),
            Error::Compiler(msg) => write!(f, "[Error][Compiler] {}", msg),
            Error::Placer(msg) => write!(f, "[Error][Placer] {}", msg),
            Error::Opt(msg) => write!(f, "[Error][Opt] {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IRParse(pest_consume::Error<irparser::Rule>),
    AsmParse(pest_consume::Error<asmparser::Rule>),
    XLParse(pest_consume::Error<xlparser::Rule>),
    TDLParse(pest_consume::Error<tdlparser::Rule>),
    Conversion(String),
    Type(String),
    TryFromInt(TryFromIntError),
    ParseInt(ParseIntError),
    Compiler(String),
    Placer(String),
    Opt(String),
}

impl Error {
    pub fn new_conv_error(msg: &str) -> Self {
        Error::Conversion(msg.to_string())
    }
    pub fn new_compiler_error(msg: &str) -> Self {
        Error::Compiler(msg.to_string())
    }
    pub fn new_placer_error(msg: &str) -> Self {
        Error::Placer(msg.to_string())
    }
    pub fn new_opt_error(msg: &str) -> Self {
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

impl From<pest_consume::Error<asmparser::Rule>> for Error {
    fn from(e: pest_consume::Error<asmparser::Rule>) -> Self {
        Error::AsmParse(e)
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

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}
