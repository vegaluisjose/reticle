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

impl TryFrom<ExprTerm> for Id {
    type Error = Error;
    fn try_from(term: ExprTerm) -> Result<Self, Self::Error> {
        match term {
            ExprTerm::Var(n, _) => Ok(n.to_string()),
            _ => Err(Error::new_conv_error("not a term variable")),
        }
    }
}

impl TryFrom<ExprTup> for Vec<i64> {
    type Error = Error;
    fn try_from(tup: ExprTup) -> Result<Self, Self::Error> {
        let mut val: Vec<i64> = Vec::new();
        for term in tup.term() {
            val.push(i64::try_from(term.clone())?)
        }
        Ok(val)
    }
}

// impl TryFrom<ExprTup> for Vec<Id> {
//     type Error = Error;
//     fn try_from(tup: ExprTup) -> Result<Self, Self::Error> {
//         let mut id: Vec<Id> = Vec::new();
//         for term in tup.term() {
//             id.push(Id::try_from(term.clone())?)
//         }
//         Ok(id)
//     }
// }