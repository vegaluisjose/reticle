use crate::lang::ast::{DataType, Expr, Id, Loc, PlacedOp};

pub type DagTy = DataType;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DagOp {
    Any,
    Ref,
    Reg,
    Add,
    Sub,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DagLoc {
    Any,
    Unknown,
    Lut,
    Lum,
    Dsp,
    Ram,
    Ref(Id),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DagInstr {
    pub loc: DagLoc,
    pub ty: DagTy,
    pub op: DagOp,
}

impl DagOp {
    pub fn from_expr(input: &Expr) -> DagOp {
        match input {
            Expr::Ref(_) => DagOp::Ref,
            _ => panic!("Error: only reference conversion supported"),
        }
    }

    pub fn from_placed_op(input: &PlacedOp) -> DagOp {
        match input {
            PlacedOp::Reg => DagOp::Reg,
            PlacedOp::Add => DagOp::Add,
            PlacedOp::Sub => DagOp::Sub,
            PlacedOp::Mul => DagOp::Mul,
            _ => panic!("Error: op not supported"),
        }
    }

    pub fn to_placed_op(&self) -> PlacedOp {
        match self {
            DagOp::Reg => PlacedOp::Reg,
            DagOp::Add => PlacedOp::Add,
            DagOp::Sub => PlacedOp::Sub,
            DagOp::Mul => PlacedOp::Mul,
            _ => panic!("Error: DagOp conversion not supported"),
        }
    }
}

impl DagLoc {
    pub fn from_loc(input: &Loc) -> DagLoc {
        match input {
            Loc::Unknown => DagLoc::Unknown,
            Loc::Lut => DagLoc::Lut,
            Loc::Lum => DagLoc::Lum,
            Loc::Dsp => DagLoc::Dsp,
            Loc::Ram => DagLoc::Ram,
            Loc::Ref(n) => DagLoc::Ref(n.to_string()),
        }
    }

    pub fn to_loc(&self) -> Loc {
        match self {
            DagLoc::Unknown => Loc::Unknown,
            DagLoc::Lut => Loc::Lut,
            DagLoc::Lum => Loc::Lum,
            DagLoc::Dsp => Loc::Dsp,
            DagLoc::Ram => Loc::Ram,
            DagLoc::Ref(n) => Loc::Ref(n.to_string()),
            _ => panic!("Error: Loc conversion not supported"),
        }
    }
}

impl DagInstr {
    pub fn new(op: DagOp, ty: DagTy, loc: DagLoc) -> DagInstr {
        DagInstr {
            loc: loc,
            ty: ty,
            op: op,
        }
    }
}
