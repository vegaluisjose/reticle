use crate::ir::ast::*;

impl From<Vec<ExprTerm>> for ExprTup {
    fn from(term: Vec<ExprTerm>) -> Self {
        ExprTup { term }
    }
}

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
            Expr::Tup(t) => t.term().clone(),
            Expr::Term(t) => vec![t],
        }
    }
}

impl From<Expr> for TermMap {
    fn from(expr: Expr) -> Self {
        let mut map = TermMap::new();
        match expr {
            Expr::Tup(t) => {
                for t in t.term() {
                    if let Some(id) = t.id() {
                        map.insert(id, t.clone());
                    }
                }
            }
            Expr::Term(t) => {
                if let Some(id) = t.id() {
                    map.insert(id, t);
                }
            }
        }
        map
    }
}

impl From<Expr> for ExprSet {
    fn from(expr: Expr) -> Self {
        let mut set = ExprSet::new();
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

impl From<Def> for InstrMap {
    fn from(input: Def) -> Self {
        let mut imap = InstrMap::new();
        for instr in input.body() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    imap.insert(id, instr.clone());
                }
            }
        }
        imap
    }
}
