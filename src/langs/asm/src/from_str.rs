use crate::ast::*;
use crate::errors::Error;
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
        if OpWire::from_str(input).is_ok() {
            let err = format!(
                "Error: {} is a wire operation and cannot be an asm operation",
                input
            );
            Err(Error::new_conv_error(&err))
        } else {
            Ok(OpAsm::Op(input.to_string()))
        }
    }
}

impl FromStr for OpCoord {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid coordinate operation", input);
        match input {
            "+" => Ok(OpCoord::Add),
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}
