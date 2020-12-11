use crate::ir::ast as ir;
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type WireOp = ir::WireOp;
pub type InstrWire = ir::InstrWire;
pub type Sig = ir::Sig;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegOp {
    Fdre,
    Fdse,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DspOp {
    Add,
    MulAdd,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DspOpt {
    Ra,
    Rb,
    Rc,
    Rd,
    Rp,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegBel {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrReg {
    pub op: RegOp,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrDsp {
    pub op: DspOp,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Reg(InstrReg),
    Dsp(InstrDsp),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Def {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Desc {
    pub def: HashMap<Id, Def>,
}
