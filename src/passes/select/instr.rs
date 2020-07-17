use crate::lang::ast;
use std::fmt;
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

    pub fn from_ast_expr(expr: &ast::Expr) -> Instr {
        let op = Op::Inp;
        let ty = expr.ty().clone();
        let loc = Loc::Var;
        Instr::new(op, ty, loc)
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

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Op::Any => "any",
            Op::Inp => "inp",
            Op::Reg => "reg",
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Loc::Any => "any".to_string(),
            Loc::Var => "??".to_string(),
            Loc::Lut => "lut".to_string(),
            Loc::Lum => "lum".to_string(),
            Loc::Dsp => "dsp".to_string(),
            Loc::Ram => "ram".to_string(),
            Loc::Ref(n) => format!("loc({})", n),
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.op, self.ty, self.loc)
    }
}

// impl Op {
//     pub fn from_expr(input: &Expr) -> Op {
//         match input {
//             Expr::Ref(_) => Op::Ref,
//             _ => panic!("Error: only reference conversion supported"),
//         }
//     }

//     pub fn from_placed_op(input: &PlacedOp) -> Op {
//         match input {
//             PlacedOp::Reg => Op::Reg,
//             PlacedOp::Add => Op::Add,
//             PlacedOp::Sub => Op::Sub,
//             PlacedOp::Mul => Op::Mul,
//             _ => panic!("Error: op not supported"),
//         }
//     }

//     pub fn to_placed_op(&self) -> PlacedOp {
//         match self {
//             Op::Reg => PlacedOp::Reg,
//             Op::Add => PlacedOp::Add,
//             Op::Sub => PlacedOp::Sub,
//             Op::Mul => PlacedOp::Mul,
//             _ => panic!("Error: Op conversion not supported"),
//         }
//     }
// }

// impl Loc {
//     pub fn from_loc(input: &Loc) -> Loc {
//         match input {
//             Loc::Unknown => Loc::Unknown,
//             Loc::Lut => Loc::Lut,
//             Loc::Lum => Loc::Lum,
//             Loc::Dsp => Loc::Dsp,
//             Loc::Ram => Loc::Ram,
//             Loc::Ref(n) => Loc::Ref(n.to_string()),
//         }
//     }

//     pub fn to_loc(&self) -> Loc {
//         match self {
//             Loc::Unknown => Loc::Unknown,
//             Loc::Lut => Loc::Lut,
//             Loc::Lum => Loc::Lum,
//             Loc::Dsp => Loc::Dsp,
//             Loc::Ram => Loc::Ram,
//             Loc::Ref(n) => Loc::Ref(n.to_string()),
//             _ => panic!("Error: Loc conversion not supported"),
//         }
//     }
// }
