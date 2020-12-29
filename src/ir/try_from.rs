use crate::ir::ast::*;
use crate::util::errors::Error;
use std::convert::TryFrom;

impl TryFrom<ExprTerm> for i64 {
    type Error = Error;
    fn try_from(term: ExprTerm) -> Result<Self, Self::Error> {
        match term {
            ExprTerm::Val(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a term value")),
        }
    }
}
