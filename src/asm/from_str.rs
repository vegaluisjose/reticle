use crate::asm::ast::*;
use crate::util::errors::Error;
use std::str::FromStr;

impl FromStr for ExprCoord {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let num = input.parse::<u64>();
        match num {
            Ok(n) => Ok(ExprCoord::Val(n)),
            Err(_) if input == "??" => Ok(ExprCoord::Any),
            _ => Ok(ExprCoord::Var(input.to_string())),
        }
    }
}

impl FromStr for OpAsm {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(OpAsm::Op(input.to_string()))
    }
}

impl FromStr for OpCoord {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid coordinate operation", input);
        match input {
            "+" => Ok(OpCoord::Add),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}
