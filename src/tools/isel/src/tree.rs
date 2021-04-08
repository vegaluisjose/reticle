use crate::errors::Error;
use asm::ast as asm;
use ir::ast as ir;
use pat::ast as pat;
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeOp {
    Wire(ir::OpWire),
    Comp(ir::OpPrim),
    Inp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub index: u64,
    pub id: ir::Id,
    pub ty: ir::Ty,
    pub op: NodeOp,
    pub attr: ir::Expr,
    pub prim: ir::Prim,
    pub cost: u64,
    pub staged: bool,
    pub committed: bool,
    pub pat: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tree {
    pub index: u64,
    pub node: HashMap<u64, Node>,
    pub edge: HashMap<u64, Vec<u64>>,
}

pub fn tree_from_pat_target(prim: &str) {
    let filename = format!("{}_pat.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    let target = pat::Target::deserialize_from_file(path);
    println!("{}", target);
}

pub fn select(_: &ir::Prog) -> Result<asm::Prog, Error> {
    tree_from_pat_target("lut");
    let prog = asm::Prog::default();
    Ok(prog)
}
