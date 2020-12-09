use crate::asm::ast::ExprCoord;
use regex::Regex;
use std::str::FromStr;

impl FromStr for ExprCoord {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re_lit = Regex::new(r"^([[:digit:]]+)$").unwrap();
        if input == "??" {
            Ok(ExprCoord::Hole)
        } else if re_lit.is_match(input) {
            Ok(ExprCoord::Lit(input.parse::<u32>().unwrap()))
        } else {
            Ok(ExprCoord::Var(input.to_string()))
        }
    }
}
