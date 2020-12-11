use crate::ir::ast as ir;
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type OpWire = ir::OpWire;
pub type InstrWire = ir::InstrWire;
pub type Sig = ir::Sig;
pub type OptMap = HashMap<Opt, OptVal>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpReg {
    Fdre,
    Fdse,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpDsp {
    Add,
    MulAdd,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OptDsp {
    Ra,
    Rb,
    Rc,
    Rd,
    Rp,
    Rm,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Opt {
    Dsp(OptDsp),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OptVal {
    UInt(u32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BelReg {
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
    pub op: OpReg,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrDsp {
    pub op: OpDsp,
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
