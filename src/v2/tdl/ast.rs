use crate::v2::il::ast as il;
use std::collections::HashMap;

pub type Id = il::Id;
pub type Ty = il::Ty;
pub type Prim = il::Prim;
pub type ExprTup = il::ExprTup;
pub type Expr = il::Expr;
pub type WireOp = il::WireOp;
pub type InstrWire = il::InstrWire;
pub type InstrComp = il::InstrComp;
pub type Sig = il::Sig;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Desc {
    pub def: HashMap<Id, Def>,
}
