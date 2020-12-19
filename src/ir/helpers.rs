use crate::ir::ast::*;
use std::collections::HashMap;

impl Ty {
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

impl ExprTup {
    pub fn expr(&self) -> &Vec<Expr> {
        &self.expr
    }
    pub fn is_empty(&self) -> bool {
        self.expr.is_empty()
    }
}

#[allow(clippy::match_like_matches_macro)]
impl Expr {
    pub fn is_tup(&self) -> bool {
        match self {
            Expr::Tup(_) => true,
            _ => false,
        }
    }
    pub fn tup(&self) -> Option<&ExprTup> {
        match self {
            Expr::Tup(e) => Some(e),
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
}

impl Prog {
    pub fn def(&self) -> &HashMap<Id, Def> {
        &self.def
    }
    pub fn get(&self, name: &str) -> &Def {
        self.def.get(name).expect("Error: function not found")
    }
    pub fn insert(&mut self, name: &str, def: Def) {
        self.def.insert(name.to_string(), def);
    }
}
