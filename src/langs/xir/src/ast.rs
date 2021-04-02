use asm::ast as asm;
use ir::ast as ir;
use serde::{Deserialize, Serialize};

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type OpCoord = asm::OpCoord;
pub type ExprTerm = ir::ExprTerm;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type ExprCoord = asm::ExprCoord;
pub type Sig = ir::Sig;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum OpBasc {
    Id,
    Gnd,
    Vcc,
    Ext,
    Cat,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum OpMach {
    Lut1,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
    Fdre,
    Fdse,
    Carry,
    Mul,
    MulAdd,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum BelCarry {
    Carry8,
    Carry4,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum BelDsp {
    Alu,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Bel {
    Lut(BelLut),
    Reg(BelReg),
    Carry(BelCarry),
    Dsp(BelDsp),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Loc {
    pub bel: Bel,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InstrBasc {
    pub op: OpBasc,
    pub attr: Expr,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InstrMach {
    pub op: OpMach,
    pub attr: Expr,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: Option<Loc>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Instr {
    Basc(InstrBasc),
    Mach(InstrMach),
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
}
