use crate::ast::*;
use crate::errors::Error;
use std::convert::TryFrom;
use std::convert::TryInto;

impl TryFrom<ExprTerm> for i64 {
    type Error = Error;
    fn try_from(term: ExprTerm) -> Result<Self, Self::Error> {
        match term {
            ExprTerm::Val(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a term value")),
        }
    }
}

impl TryFrom<ExprTerm> for i32 {
    type Error = Error;
    fn try_from(term: ExprTerm) -> Result<Self, Self::Error> {
        match term {
            ExprTerm::Val(n) => Ok(i32::try_from(n)?),
            _ => Err(Error::new_conv_error("not a term value")),
        }
    }
}

impl TryFrom<ExprTerm> for Id {
    type Error = Error;
    fn try_from(term: ExprTerm) -> Result<Self, Self::Error> {
        match term {
            ExprTerm::Var(n, _) => Ok(n),
            ExprTerm::Any => Ok(String::new()),
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

impl TryFrom<ExprTup> for Vec<i32> {
    type Error = Error;
    fn try_from(tup: ExprTup) -> Result<Self, Self::Error> {
        let mut val: Vec<i32> = Vec::new();
        for term in tup.term() {
            val.push(i32::try_from(term.clone())?)
        }
        Ok(val)
    }
}

impl TryFrom<ExprTup> for Vec<Id> {
    type Error = Error;
    fn try_from(tup: ExprTup) -> Result<Self, Self::Error> {
        let mut id: Vec<Id> = Vec::new();
        for term in tup.term() {
            id.push(Id::try_from(term.clone())?)
        }
        Ok(id)
    }
}

impl TryFrom<Expr> for Vec<i64> {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        match expr {
            Expr::Term(term) => Ok(vec![i64::try_from(term)?]),
            Expr::Tup(tup) => Ok(tup.try_into()?),
        }
    }
}

impl TryFrom<Expr> for Vec<i32> {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        match expr {
            Expr::Term(term) => Ok(vec![i32::try_from(term)?]),
            Expr::Tup(tup) => Ok(tup.try_into()?),
        }
    }
}

impl TryFrom<Expr> for Vec<Id> {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        match expr {
            Expr::Term(term) => Ok(vec![Id::try_from(term)?]),
            Expr::Tup(tup) => Ok(tup.try_into()?),
        }
    }
}

impl TryFrom<Instr> for InstrWire {
    type Error = Error;
    fn try_from(instr: Instr) -> Result<Self, Self::Error> {
        match instr {
            Instr::Wire(instr) => Ok(instr),
            _ => Err(Error::new_conv_error("not a wire instruction")),
        }
    }
}
