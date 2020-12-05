use crate::v2::il::ast::*;
use std::collections::HashMap;

impl ExprTup {
    pub fn exprs(&self) -> &Vec<Expr> {
        &self.exprs
    }
    pub fn is_empty(&self) -> bool {
        self.exprs.is_empty()
    }
}

impl Expr {
    pub fn is_tup(&self) -> bool {
        match self {
            Expr::Tup(_) => true,
            _ => false,
        }
    }
    pub fn tup(&self) -> &ExprTup {
        match self {
            Expr::Tup(e) => e,
            _ => panic!("Error: expr is not tuple"),
        }
    }
}

impl Sig {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn inputs(&self) -> &Expr {
        &self.inputs
    }
    pub fn outputs(&self) -> &Expr {
        &self.outputs
    }
}

impl Def {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
}

impl Prog {
    pub fn defs(&self) -> &HashMap<Id, Def> {
        &self.defs
    }
    pub fn get(&self, name: &str) -> &Def {
        self.defs.get(name).expect("Error: function not found")
    }
    pub fn insert(&mut self, name: &str, def: Def) {
        self.defs.insert(name.to_string(), def);
    }
}
