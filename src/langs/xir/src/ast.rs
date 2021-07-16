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
pub type Mem = mmap::Mem;
// pub type Mmap = mmap::Mmap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum OpBasc {
    Id,
    Gnd,
    Vcc,
    Ext,
    Cat,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum OpMach {
    Lut1,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
    Fdre,
    Fdse,
    CarryAdd,
    CarrySub,
    VecAddRegA,
    VecAdd,
    VecSub,
    VecMul,
    Mul,
    MulAdd,
    MulAddRegA,
    MulAddRegACi,
    MulAddRegACo,
    MulAddRegACio,
    Lram,
    Bram,
    Lrom,
    Brom,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum BelCarry {
    Carry8,
    Carry4,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum BelDsp {
    Alu,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum BelRamb {
    L,
    U,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Bel {
    Lut(BelLut),
    Reg(BelReg),
    Carry(BelCarry),
    Dsp(BelDsp),
    Ramb(BelRamb),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct Loc {
    pub bel: Bel,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct InstrBasc {
    pub op: OpBasc,
    pub attr: Expr,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct InstrMach {
    pub op: OpMach,
    pub attr: Expr,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: Option<Loc>,
    pub mem: Option<Mem>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Instr {
    Basc(InstrBasc),
    Mach(InstrMach),
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug, Eq)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
}
