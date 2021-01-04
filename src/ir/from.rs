use crate::ir::ast::*;
use std::collections::HashSet;

impl From<ExprTup> for Expr {
    fn from(tup: ExprTup) -> Self {
        Expr::Tup(tup)
    }
}

impl From<ExprTerm> for Expr {
    fn from(term: ExprTerm) -> Self {
        Expr::Term(term)
    }
}

impl From<InstrCall> for Instr {
    fn from(instr: InstrCall) -> Self {
        Instr::Call(instr)
    }
}

impl From<InstrWire> for Instr {
    fn from(instr: InstrWire) -> Self {
        Instr::Wire(instr)
    }
}

impl From<InstrComp> for Instr {
    fn from(instr: InstrComp) -> Self {
        Instr::Comp(instr)
    }
}

impl From<Expr> for Vec<ExprTerm> {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Tup(tup) => tup.term().clone(),
            Expr::Term(term) => vec![term.clone()],
        }
    }
}

impl From<Expr> for HashSet<Expr> {
    fn from(expr: Expr) -> Self {
        let mut set: HashSet<Expr> = HashSet::new();
        match &expr {
            Expr::Tup(tup) => {
                for term in tup.term() {
                    set.insert(Expr::from(term.clone()));
                }
            }
            Expr::Term(term) => {
                set.insert(Expr::from(term.clone()));
            }
        }
        set
    }
}
