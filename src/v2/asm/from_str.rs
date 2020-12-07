use crate::util::errors::Error;
use crate::v2::asm::ast::*;
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
