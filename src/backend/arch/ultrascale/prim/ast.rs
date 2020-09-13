use std::collections::HashMap;

pub type Id = String;
pub type PortMap = HashMap<String, Expr>;
pub type ParamMap = HashMap<String, i64>;

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(String),
    Index(String, u32),
}

#[derive(Clone, Debug)]
pub struct Slice {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Debug)]
pub enum BelTy {
    A6LUT,
    FF,
}

#[derive(Clone, Debug)]
pub struct Bel {
    pub letter: Letter,
    pub ty: BelTy,
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub slice: Slice,
    pub bel: Bel,
}

#[derive(Clone, Debug)]
pub enum LutTy {
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
}

#[derive(Clone, Debug)]
pub struct Lut {
    pub ty: LutTy,
    pub id: Id,
    pub init: String,
    pub inputs: Vec<Expr>,
    pub output: Expr,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug)]
pub enum RegTy {
    Fdre,
    Fdse,
}

#[derive(Clone, Debug)]
pub struct Reg {
    pub ty: RegTy,
    pub id: Id,
    pub inputs: PortMap,
    pub outputs: PortMap,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DspScalarOp {
    MulAdd,
}

#[derive(Clone, Debug)]
pub struct DspScalar {
    pub op: DspScalarOp,
    pub id: Id,
    pub widths: HashMap<String, u64>,
    pub inputs: PortMap,
    pub outputs: PortMap,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DspVectorOp {
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub struct DspVector {
    pub op: DspVectorOp,
    pub id: Id,
    pub params: ParamMap,
    pub inputs: PortMap,
    pub outputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Vcc {
    pub id: Id,
    pub outputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Gnd {
    pub id: Id,
    pub outputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Const {
    pub id: Id,
    pub params: ParamMap,
    pub inputs: PortMap,
}
