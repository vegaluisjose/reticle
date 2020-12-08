use crate::v2::ir::ast as ir;
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type WireOp = ir::WireOp;
pub type CompOp = ir::CompOp;
pub type InstrWire = ir::InstrWire;
pub type InstrComp = ir::InstrComp;
pub type Sig = ir::Sig;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Comp(InstrComp),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Def {
    pub prim: Prim,
    pub area: u32,
    pub lat: u32,
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Desc {
    pub def: HashMap<Id, Def>,
}
