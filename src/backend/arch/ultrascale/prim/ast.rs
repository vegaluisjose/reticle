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
    pub id: String,
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
    pub id: String,
    pub clock: Expr,
    pub reset: Expr,
    pub en: Expr,
    pub input: Expr,
    pub output: Expr,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug)]
pub enum DspOp {
    Add,
    Sub,
    Mul,
}

#[derive(Clone, Debug)]
pub enum DspTy {
    Scalar,
    Vector(u64),
}

#[derive(Clone, Debug)]
pub struct Dsp {
    pub ty: DspTy,
    pub width: u64,
    pub op: DspOp,
    pub id: String,
    pub clock: Expr,
    pub reset: Expr,
    pub en: Expr,
    pub left: Expr,
    pub right: Expr,
    pub output: Expr,
}

#[derive(Clone, Debug)]
pub struct Vcc {
    pub id: String,
    pub output: Expr,
}

#[derive(Clone, Debug)]
pub struct Gnd {
    pub id: String,
    pub output: Expr,
}

#[derive(Clone, Debug)]
pub struct Const {
    pub id: String,
    pub gnd: Expr,
    pub vcc: Expr,
    pub width: u64,
    pub value: i64,
}
