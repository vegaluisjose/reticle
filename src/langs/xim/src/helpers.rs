use crate::ast::*;
use bincode::{deserialize_from, serialize_into};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

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
    pub fn area(&self) -> u64 {
        self.area
    }
    pub fn perf(&self) -> u64 {
        self.perf
    }
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }
    pub fn set_area(&mut self, area: u64) {
        self.area = area;
    }
    pub fn set_perf(&mut self, perf: u64) {
        self.perf = perf;
    }
}

impl Imp {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn area(&self) -> u64 {
        self.sig.area
    }
    pub fn perf(&self) -> u64 {
        self.sig.perf
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
    pub fn imp(&self) -> &HashMap<Id, Imp> {
        &self.imp
    }
    pub fn get(&self, name: &str) -> Option<&Imp> {
        self.imp.get(name)
    }
    pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) {
        let file = File::create(path).expect("Error: cannot create the bin file");
        let mut buf = BufWriter::new(file);
        serialize_into(&mut buf, &self).expect("Error: cannot serialize");
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Imp> {
        self.imp.get_mut(name)
    }
    pub fn insert(&mut self, name: &str, imp: Imp) -> Option<Imp> {
        self.imp.insert(name.to_string(), imp)
    }
    pub fn extend(&mut self, target: Target) {
        self.imp.extend(target.imp().clone());
    }
}
