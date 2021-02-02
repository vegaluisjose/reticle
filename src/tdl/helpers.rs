use crate::tdl::ast::*;
use std::collections::HashMap;

impl PatInstr {
    pub fn dst(&self) -> &Expr {
        match self {
            PatInstr::Wire(instr) => instr.dst(),
            PatInstr::Comp(instr) => instr.dst(),
        }
    }
    pub fn attr(&self) -> &Expr {
        match self {
            PatInstr::Wire(instr) => instr.attr(),
            PatInstr::Comp(instr) => instr.attr(),
        }
    }
    pub fn arg(&self) -> &Expr {
        match self {
            PatInstr::Wire(instr) => instr.arg(),
            PatInstr::Comp(instr) => instr.arg(),
        }
    }
    pub fn set_dst(&mut self, dst: Expr) {
        match self {
            PatInstr::Wire(instr) => instr.set_dst(dst),
            PatInstr::Comp(instr) => instr.set_dst(dst),
        }
    }
    pub fn set_arg(&mut self, arg: Expr) {
        match self {
            PatInstr::Wire(instr) => instr.set_arg(arg),
            PatInstr::Comp(instr) => instr.set_arg(arg),
        }
    }
}

impl PatSig {
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

impl ImpSig {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
    pub fn input(&self) -> &Expr {
        &self.input
    }
    pub fn output(&self) -> &Expr {
        &self.output
    }
}

impl Pat {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn area(&self) -> u64 {
        self.sig.area()
    }
    pub fn lat(&self) -> u64 {
        self.sig.lat()
    }
    pub fn sig(&self) -> &PatSig {
        &self.sig
    }
    pub fn input(&self) -> &Expr {
        self.sig.input()
    }
    pub fn output(&self) -> &Expr {
        self.sig.output()
    }
    pub fn prim(&self) -> &Prim {
        self.sig.prim()
    }
    pub fn body(&self) -> &Vec<PatInstr> {
        &self.body
    }
    pub fn set_body(&mut self, body: Vec<PatInstr>) {
        self.body = body;
    }
    pub fn set_sig(&mut self, sig: PatSig) {
        self.sig = sig;
    }
}

impl Imp {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn sig(&self) -> &ImpSig {
        &self.sig
    }
    pub fn input(&self) -> &Expr {
        self.sig.input()
    }
    pub fn output(&self) -> &Expr {
        self.sig.output()
    }
    pub fn body(&self) -> &Vec<ImpInstr> {
        &self.body
    }
    pub fn set_body(&mut self, body: Vec<ImpInstr>) {
        self.body = body;
    }
    pub fn set_sig(&mut self, sig: ImpSig) {
        self.sig = sig;
    }
}

impl Target {
    pub fn pat(&self) -> &HashMap<Id, Pat> {
        &self.pat
    }
    pub fn imp(&self) -> &HashMap<Id, Imp> {
        &self.imp
    }
    pub fn get_pat(&self, name: &str) -> Option<&Pat> {
        self.pat.get(name)
    }
    pub fn get_imp(&self, name: &str) -> Option<&Imp> {
        self.imp.get(name)
    }
    pub fn add_pat(&mut self, name: &str, pat: Pat) {
        self.pat.insert(name.to_string(), pat);
    }
    pub fn add_imp(&mut self, name: &str, imp: Imp) {
        self.imp.insert(name.to_string(), imp);
    }
}
