pub mod default;
pub mod helpers;

// use crate::errors::Error;
// use asm::ast as asm;
// use pat::ast as pat;
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tree {
    pub index: u64,
    pub node: HashMap<u64, Node>,
    pub edge: HashMap<u64, Vec<u64>>,
}
