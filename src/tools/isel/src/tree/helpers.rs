use crate::tree::*;
use ir::ast as ir;
use std::collections::HashMap;

impl Node {
    pub fn index(&self) -> u64 {
        self.index
    }
    pub fn id(&self) -> Id {
        self.id.to_string()
    }
    pub fn ty(&self) -> &Ty {
        &self.ty
    }
    pub fn op(&self) -> &NodeOp {
        &self.op
    }
    pub fn is_inp(&self) -> bool {
        matches!(self.op, NodeOp::Inp)
    }
    pub fn is_wire(&self) -> bool {
        matches!(self.op, NodeOp::Wire(_))
    }
    pub fn is_prim(&self) -> bool {
        matches!(self.op, NodeOp::Prim(_))
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn cost(&self) -> u64 {
        self.cost
    }
    pub fn pat(&self) -> Option<&String> {
        self.pat.as_ref()
    }
    pub fn is_staged(&self) -> bool {
        self.staged
    }
    pub fn is_committed(&self) -> bool {
        self.committed
    }
    pub fn is_free(&self) -> bool {
        !(self.staged | self.committed)
    }
    pub fn set_index(&mut self, index: u64) {
        self.index = index;
    }
    pub fn set_cost(&mut self, cost: u64) {
        self.cost = cost;
    }
    pub fn stage(&mut self) {
        self.staged = true;
    }
    pub fn commit(&mut self) {
        self.committed = true;
    }
    pub fn set_pat(&mut self, name: &str) {
        self.pat = Some(name.to_string());
    }
    pub fn clear_staged(&mut self) {
        self.staged = false;
    }
    pub fn clear_commit(&mut self) {
        self.committed = false;
    }
    pub fn clear_pat(&mut self) {
        self.pat = None;
    }
}

pub fn tree_roots_from_def(def: &ir::Def) -> Vec<ir::Id> {
    let mut count: HashMap<ir::Id, u64> = HashMap::new();
    // store compute instructions
    for instr in def.body() {
        if instr.is_prim() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    count.insert(id, 0);
                }
            }
        }
    }
    // calculate the number of times compute instructions are used
    for instr in def.body() {
        let arg: Vec<ir::ExprTerm> = instr.arg().clone().into();
        for e in arg {
            if let Some(id) = e.id() {
                if let Some(val) = count.get_mut(&id) {
                    *val += 1;
                }
            }
        }
    }
    let mut root: Vec<ir::Id> = Vec::new();
    // a node is a root if it is used more than once
    for (k, v) in count {
        if v > 1 {
            root.push(k);
        }
    }
    // add outputs as roots
    let output: Vec<ir::ExprTerm> = def.output().clone().into();
    for e in output {
        if let Some(id) = e.id() {
            root.push(id);
        }
    }
    root
}
