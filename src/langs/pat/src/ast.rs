use ir::ast as ir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTerm = ir::ExprTerm;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type OpWire = ir::OpWire;
pub type OpPrim = ir::OpPrim;
pub type InstrWire = ir::InstrWire;
pub type InstrPrim = ir::InstrPrim;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Prim(InstrPrim),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct Sig {
    pub id: Id,
    pub input: Expr,
    pub output: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash, Default)]
pub struct Pat {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Default)]
pub struct Target {
    pub pat: HashMap<Id, Pat>,
}
