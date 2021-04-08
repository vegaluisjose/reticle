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
pub type Sig = ir::Sig;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Prim(InstrPrim),
}

pub type InstrMap = HashMap<String, Instr>;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash, Default)]
pub struct Pat {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Default)]
pub struct Target {
    pub pat: HashMap<Id, Pat>,
}
