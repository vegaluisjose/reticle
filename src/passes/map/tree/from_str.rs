use crate::lang::ast::PrimOp;
use crate::passes::map::tree::TreeOp;
use std::str::FromStr;

impl FromStr for TreeOp {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(TreeOp::from(PrimOp::from_str(input)?))
    }
}
