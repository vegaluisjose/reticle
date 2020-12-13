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

#[derive(Clone, Debug)]
pub enum OpReg {
    Fdre,
    Fdse,
}

#[derive(Clone, Debug)]
pub enum OpDsp {
    Add,
    MulAdd,
}

#[derive(Clone, Debug)]
pub enum OpCarry {
    Carry8,
}

#[derive(Clone, Debug)]
pub enum OpLut {
    Lut1,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
}

#[derive(Clone, Debug)]
pub enum OptDsp {
    Ra,
    Rb,
    Rc,
    Rm,
    Rp,
}

#[derive(Clone, Debug)]
pub enum Opt {
    Dsp(OptDsp),
}

#[derive(Clone, Debug)]
pub enum OptVal {
    UInt(u64),
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
pub struct LocLut {
    pub bel: BelLut,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug)]
pub struct LocReg {
    pub bel: BelReg,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug)]
pub struct LocCarry {
    pub bel: BelCarry,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug)]
pub struct LocDsp {
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug)]
pub struct InstrReg {
    pub op: OpReg,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocReg,
}

#[derive(Clone, Debug)]
pub struct InstrLut {
    pub op: OpLut,
    pub opt: OptMap,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocLut,
}

#[derive(Clone, Debug)]
pub struct InstrCarry {
    pub op: OpCarry,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocCarry,
}

#[derive(Clone, Debug)]
pub struct InstrDsp {
    pub op: OpDsp,
    pub opt: OptMap,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: LocDsp,
}

#[derive(Clone, Debug)]
pub enum Instr {
    Wire(InstrWire),
    Reg(InstrReg),
    Dsp(InstrDsp),
    Lut(InstrLut),
    Carry(InstrCarry),
}

#[derive(Clone, Debug)]
pub struct Def {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Clone, Debug, Default)]
pub struct Desc {
    pub def: HashMap<Id, Def>,
}
