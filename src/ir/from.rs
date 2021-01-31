use crate::ir::ast::*;
use crate::tdl::ast::PatInstr;
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

impl From<PatInstr> for Instr {
    fn from(instr: PatInstr) -> Self {
        match instr {
            PatInstr::Wire(instr) => Instr::Wire(instr),
            PatInstr::Comp(instr) => Instr::Comp(instr),
        }
    }
}

impl From<Expr> for Vec<ExprTerm> {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Tup(t) => t.term().clone(),
            Expr::Term(t) => vec![t],
        }
    }
}

impl From<Expr> for HashSet<Expr> {
    fn from(expr: Expr) -> Self {
        let mut set: HashSet<Expr> = HashSet::new();
        match expr {
            Expr::Tup(tup) => {
                for t in tup.term() {
                    set.insert(Expr::from(t.clone()));
                }
            }
            Expr::Term(t) => {
                set.insert(Expr::from(t));
            }
        }
        set
    }
}
