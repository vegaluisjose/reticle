pub mod default;
pub mod display;
pub mod from;
pub mod helpers;
pub mod try_from;

use ir::ast as ir;
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTerm = ir::ExprTerm;
pub type Expr = ir::Expr;
pub type OpWire = ir::OpWire;
pub type OpPrim = ir::OpPrim;
pub type Def = ir::Def;
pub type InstrWire = ir::InstrWire;
pub type InstrPrim = ir::InstrPrim;
pub type Instr = ir::Instr;
pub type InstrMap = ir::InstrMap;
pub type TermMap = ir::TermMap;
pub type Prog = ir::Prog;
pub type TreeMap = HashMap<String, Tree>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeOp {
    Wire(OpWire),
    Prim(OpPrim),
    Inp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub index: u64,
    pub id: Id,
    pub ty: Ty,
    pub op: NodeOp,
    pub attr: Expr,
    pub prim: Prim,
    pub cost: u64,
    pub staged: bool,
    pub committed: bool,
    pub pat: Option<String>,
    pub pat_prim: Prim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tree {
    pub index: u64,
    pub node: HashMap<u64, Node>,
    pub edge: HashMap<u64, Vec<u64>>,
}
