use crate::asm::ast as asm;
use crate::ir::ast as ir;
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Opt {
    RegA,
    RegB,
    RegC,
    RegM,
    RegP,
    Op,
    Table,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum OptOp {
    Add,
    MulAdd,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum OptVal {
    UInt(u64),
    Op(OptOp),
}

#[derive(Clone, Debug)]
pub enum OpMach {
    Lut1,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
    Fdre,
    Fdse,
    Dsp,
    Carry,
}

#[derive(Clone, Debug)]
pub enum BelLut {
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
}

#[derive(Clone, Debug)]
pub enum BelReg {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
}

#[derive(Clone, Debug)]
pub enum BelCarry {
    Carry8,
    Carry4,
}

#[derive(Clone, Debug)]
pub enum Bel {
    Lut(BelLut),
    Reg(BelReg),
    Carry(BelCarry),
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub bel: Option<Bel>,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug)]
pub struct InstrMach {
    pub op: OpMach,
    pub opt: OptMap,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: Loc,
}

#[derive(Clone, Debug)]
pub enum Instr {
    Wire(InstrWire),
    Mach(InstrMach),
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
}
