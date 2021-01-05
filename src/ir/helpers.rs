use crate::ir::ast::*;
use std::collections::HashMap;

impl Ty {
    pub fn width(&self) -> Option<u64> {
        match self {
            Ty::Bool => Some(1),
            Ty::UInt(w) => Some(*w),
            Ty::SInt(w) => Some(*w),
            Ty::Vector(d, _) => d.width(),
            _ => None,
        }
    }
    pub fn length(&self) -> Option<u64> {
        match self {
            Ty::Vector(_, l) => Some(*l),
            _ => None,
        }
    }
    pub fn is_vector(&self) -> bool {
        matches!(self, Ty::Vector(_, _))
    }
}

impl OpCall {
    pub fn new(op: &str) -> OpCall {
        OpCall { op: op.to_string() }
    }
    pub fn op(&self) -> String {
        self.op.to_string()
    }
}

impl ExprTerm {
    pub fn is_var(&self) -> bool {
        matches!(self, ExprTerm::Var(_, _))
    }
    pub fn val(&self) -> Option<i64> {
        match self {
            ExprTerm::Val(n) => Some(*n),
            _ => None,
        }
    }
    pub fn id(&self) -> Option<Id> {
        match self {
            ExprTerm::Var(n, _) => Some(n.to_string()),
            _ => None,
        }
    }
    pub fn ty(&self) -> Option<&Ty> {
        match self {
            ExprTerm::Var(_, ty) => Some(ty),
            _ => None,
        }
    }
    pub fn width(&self) -> Option<u64> {
        match self {
            ExprTerm::Var(_, ty) => ty.width(),
            _ => None,
        }
    }
    pub fn length(&self) -> Option<u64> {
        match self {
            ExprTerm::Var(_, ty) => ty.length(),
            _ => None,
        }
    }
    pub fn is_vector(&self) -> bool {
        match self {
            ExprTerm::Var(_, ty) => ty.is_vector(),
            _ => false,
        }
    }
}

impl ExprTup {
    pub fn term(&self) -> &Vec<ExprTerm> {
        &self.term
    }
    pub fn idx(&self, val: usize) -> &ExprTerm {
        &self.term[val]
    }
    pub fn is_empty(&self) -> bool {
        self.term.is_empty()
    }
    pub fn add_term(&mut self, term: ExprTerm) {
        self.term.push(term);
    }
}

impl Expr {
    pub fn is_tup(&self) -> bool {
        matches!(self, Expr::Tup(_))
    }
    pub fn is_term(&self) -> bool {
        matches!(self, Expr::Term(_))
    }
    pub fn tup(&self) -> Option<&ExprTup> {
        match self {
            Expr::Tup(e) => Some(e),
            _ => None,
        }
    }
    pub fn term(&self) -> Option<&ExprTerm> {
        match self {
            Expr::Term(e) => Some(e),
            _ => None,
        }
    }
    pub fn idx(&self, val: usize) -> Option<&ExprTerm> {
        match self {
            Expr::Tup(t) => Some(t.idx(val)),
            _ => None,
        }
    }
}

impl InstrCall {
    pub fn op(&self) -> &OpCall {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
}

impl InstrComp {
    pub fn op(&self) -> &OpComp {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
}

impl InstrWire {
    pub fn op(&self) -> &OpWire {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
}

impl Sig {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn input(&self) -> &Expr {
        &self.input
    }
    pub fn output(&self) -> &Expr {
        &self.output
    }
}

impl Def {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
    pub fn body_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.body
    }
}

impl Prog {
    pub fn def(&self) -> &HashMap<Id, Def> {
        &self.def
    }
    pub fn get(&self, name: &str) -> Option<&Def> {
        self.def.get(name)
    }
    pub fn insert(&mut self, name: &str, def: Def) {
        self.def.insert(name.to_string(), def);
    }
}
