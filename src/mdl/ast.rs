use crate::ir::ast as ir;
use crate::asm::ast as asm;
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprCoord = asm::ExprCoord;
pub type OpCoord = asm::OpCoord;
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
pub enum OpCarry {
    Carry8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpLut {
    Lut1,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
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
    UInt(u64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BelLut {
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
pub enum BelCarry {
    Carry8,
    Carry4,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocLut {
    pub bel: BelLut,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocReg {
    pub bel: BelReg,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocCarry {
    pub bel: BelCarry,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocDsp {
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrReg {
    pub op: OpReg,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocReg,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrLut {
    pub op: OpLut,
    pub opt: OptMap,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocLut,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrCarry {
    pub op: OpCarry,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocCarry,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrDsp {
    pub op: OpDsp,
    pub opt: OptMap,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocDsp,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Reg(InstrReg),
    Dsp(InstrDsp),
    Lut(InstrLut),
    Carry(InstrCarry),
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
