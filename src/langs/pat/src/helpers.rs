use crate::ast::*;
use bincode::{deserialize_from, serialize_into};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

impl Instr {
    pub fn is_reg(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_reg(),
            _ => false,
        }
    }
    pub fn is_prim(&self) -> bool {
        matches!(self, Instr::Prim(_))
    }
    pub fn is_wire(&self) -> bool {
        matches!(self, Instr::Wire(_))
    }
    pub fn dst(&self) -> &Expr {
        match self {
            Instr::Prim(instr) => instr.dst(),
            Instr::Wire(instr) => instr.dst(),
        }
    }
    pub fn arg(&self) -> &Expr {
        match self {
            Instr::Prim(instr) => instr.arg(),
            Instr::Wire(instr) => instr.arg(),
        }
    }
    pub fn set_dst(&mut self, dst: Expr) {
        match self {
            Instr::Prim(instr) => instr.set_dst(dst),
            Instr::Wire(instr) => instr.set_dst(dst),
        }
    }
    pub fn set_arg(&mut self, arg: Expr) {
        match self {
            Instr::Prim(instr) => instr.set_arg(arg),
            Instr::Wire(instr) => instr.set_arg(arg),
        }
    }
}

impl Pat {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn input(&self) -> &Expr {
        self.sig.input()
    }
    pub fn output(&self) -> &Expr {
        self.sig.output()
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
    pub fn set_sig(&mut self, sig: Sig) {
        self.sig = sig;
    }
    pub fn set_body(&mut self, body: Vec<Instr>) {
        self.body = body;
    }
    pub fn body_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.body
    }
}

impl Target {
    pub fn deserialize_from_file<P: AsRef<Path>>(path: P) -> Target {
        let file = File::open(path).expect("Error: cannot open the bin file");
        let mut buf = BufReader::new(file);
        deserialize_from(&mut buf).expect("Error: cannot deserialize")
    }
    pub fn pat(&self) -> &HashMap<Id, Pat> {
        &self.pat
    }
    pub fn get(&self, name: &str) -> Option<&Pat> {
        self.pat.get(name)
    }
    pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) {
        let file = File::create(path).expect("Error: cannot create the bin file");
        let mut buf = BufWriter::new(file);
        serialize_into(&mut buf, &self).expect("Error: cannot serialize");
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Pat> {
        self.pat.get_mut(name)
    }
    pub fn insert(&mut self, name: &str, pat: Pat) -> Option<Pat> {
        self.pat.insert(name.to_string(), pat)
    }
    pub fn extend(&mut self, target: Target) {
        self.pat.extend(target.pat().clone());
    }
}
