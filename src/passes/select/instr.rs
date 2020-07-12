use crate::lang::ast::{DataType, Expr, PlacedOp, Loc, Id};

pub type InstrTy = DataType;

#[derive(Clone, Debug)]
pub enum InstrOp {
    Any,
    Ref,
    Reg,
    Add,
    Sub,
    Mul,
}

#[derive(Clone, Debug)]
pub enum InstrLoc {
    Any,
    Unknown,
    Lut,
    Lum,
    Dsp,
    Ram,
    Ref(Id),
}

#[derive(Clone, Debug)]
pub struct Instr {
    pub op: InstrOp,
    pub ty: InstrTy,
    pub loc: InstrLoc,
}

impl InstrOp {
    pub fn from_expr(input: &Expr) -> InstrOp {
        match input {
            Expr::Ref(_) => InstrOp::Ref,
            _ => panic!("Error: only reference conversion supported"),
        }
    }

    pub fn from_placed_op(input: &PlacedOp) -> InstrOp {
        match input {
            PlacedOp::Reg => InstrOp::Reg,
            PlacedOp::Add => InstrOp::Add,
            PlacedOp::Sub => InstrOp::Sub,
            PlacedOp::Mul => InstrOp::Mul,
            _ => panic!("Error: op not supported"),
        }
    }
}

impl InstrLoc {
    pub fn from_loc(input: &Loc) -> InstrLoc {
        match input {
            Loc::Unknown => InstrLoc::Unknown,
            Loc::Lut => InstrLoc::Lut,
            Loc::Lum => InstrLoc::Lum,
            Loc::Dsp => InstrLoc::Dsp,
            Loc::Ram => InstrLoc::Ram,
            Loc::Ref(n) => InstrLoc::Ref(n.to_string()),
        }
    }
}

impl Instr {
    pub fn new(op: InstrOp, ty: InstrTy, loc: InstrLoc) -> Instr {
        Instr { op: op, ty: ty, loc: loc }
    }
}
