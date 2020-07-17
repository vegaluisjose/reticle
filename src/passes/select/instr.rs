use crate::lang::ast;
use std::str::FromStr;

pub type Id = ast::Id;
pub type Ty = ast::Ty;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    Any,
    Inp,
    Reg,
    Add,
    Sub,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Loc {
    Any,
    Var,
    Lut,
    Lum,
    Dsp,
    Ram,
    Ref(Id),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Instr {
    pub loc: Loc,
    pub ty: Ty,
    pub op: Op,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub name: String,
    pub cost: u32,
    pub instr: Vec<Instr>,
}

impl FromStr for Op {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.as_ref() {
            "any" => Ok(Op::Any),
            "reg" => Ok(Op::Reg),
            "add" => Ok(Op::Add),
            "sub" => Ok(Op::Sub),
            "mul" => Ok(Op::Mul),
            _ => panic!("Error: FromStr to Op conversion"),
        }
    }
}

impl FromStr for Loc {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.as_ref() {
            "any" => Ok(Loc::Any),
            "??" => Ok(Loc::Var),
            "lut" => Ok(Loc::Lut),
            "lum" => Ok(Loc::Lum),
            "dsp" => Ok(Loc::Dsp),
            "ram" => Ok(Loc::Ram),
            _ => panic!("Error: FromStr to Loc conversion"),
        }
    }
}

impl Instr {
    pub fn new(op: Op, ty: Ty, loc: Loc) -> Instr {
        Instr {
            op: op,
            ty: ty,
            loc: loc,
        }
    }
}

impl Pattern {
    pub fn new_with_cost(name: &str, cost: u32) -> Pattern {
        Pattern {
            name: name.to_string(),
            cost: cost.clone(),
            instr: Vec::new(),
        }
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.instr.push(instr);
    }
}

// impl DagOp {
//     pub fn from_expr(input: &Expr) -> DagOp {
//         match input {
//             Expr::Ref(_) => DagOp::Ref,
//             _ => panic!("Error: only reference conversion supported"),
//         }
//     }

//     pub fn from_placed_op(input: &PlacedOp) -> DagOp {
//         match input {
//             PlacedOp::Reg => DagOp::Reg,
//             PlacedOp::Add => DagOp::Add,
//             PlacedOp::Sub => DagOp::Sub,
//             PlacedOp::Mul => DagOp::Mul,
//             _ => panic!("Error: op not supported"),
//         }
//     }

//     pub fn to_placed_op(&self) -> PlacedOp {
//         match self {
//             DagOp::Reg => PlacedOp::Reg,
//             DagOp::Add => PlacedOp::Add,
//             DagOp::Sub => PlacedOp::Sub,
//             DagOp::Mul => PlacedOp::Mul,
//             _ => panic!("Error: DagOp conversion not supported"),
//         }
//     }
// }

// impl DagLoc {
//     pub fn from_loc(input: &Loc) -> DagLoc {
//         match input {
//             Loc::Unknown => DagLoc::Unknown,
//             Loc::Lut => DagLoc::Lut,
//             Loc::Lum => DagLoc::Lum,
//             Loc::Dsp => DagLoc::Dsp,
//             Loc::Ram => DagLoc::Ram,
//             Loc::Ref(n) => DagLoc::Ref(n.to_string()),
//         }
//     }

//     pub fn to_loc(&self) -> Loc {
//         match self {
//             DagLoc::Unknown => Loc::Unknown,
//             DagLoc::Lut => Loc::Lut,
//             DagLoc::Lum => Loc::Lum,
//             DagLoc::Dsp => Loc::Dsp,
//             DagLoc::Ram => Loc::Ram,
//             DagLoc::Ref(n) => Loc::Ref(n.to_string()),
//             _ => panic!("Error: Loc conversion not supported"),
//         }
//     }
// }