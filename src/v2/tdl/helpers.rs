use crate::v2::tdl::ast::*;
use std::collections::HashMap;

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
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn area(&self) -> u64 {
        self.area
    }
    pub fn lat(&self) -> u64 {
        self.lat
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

impl Desc {
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
