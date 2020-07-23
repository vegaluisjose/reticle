use crate::lang::ast;
use std::fmt;
use std::str::FromStr;

pub type Id = ast::Id;
pub type Ty = ast::Ty;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    Any,
    In,
    Reg,
    Add,
    Sub,
    Mul,
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Mux,
    Equal,
    Nequal,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Loc {
    Any,
    Hole,
    Lut,
    Lum,
    Dsp,
    Ram,
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
            "inp" => Ok(Op::In),
            "reg" => Ok(Op::Reg),
            "add" => Ok(Op::Add),
            "sub" => Ok(Op::Sub),
            "mul" => Ok(Op::Mul),
            "not" => Ok(Op::Not),
            "and" => Ok(Op::And),
            "nand" => Ok(Op::Nand),
            "or" => Ok(Op::Or),
            "nor" => Ok(Op::Nor),
            "xor" => Ok(Op::Xor),
            "xnor" => Ok(Op::Xnor),
            "mux" => Ok(Op::Mux),
            "eq" => Ok(Op::Equal),
            "neq" => Ok(Op::Nequal),
            "gt" => Ok(Op::Gt),
            "lt" => Ok(Op::Lt),
            "ge" => Ok(Op::Ge),
            "le" => Ok(Op::Le),
            _ => panic!("Error: FromStr to Op conversion"),
        }
    }
}

impl FromStr for Loc {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.as_ref() {
            "any" => Ok(Loc::Any),
            "??" => Ok(Loc::Hole),
            "lut" => Ok(Loc::Lut),
            "lum" => Ok(Loc::Lum),
            "dsp" => Ok(Loc::Dsp),
            "ram" => Ok(Loc::Ram),
            _ => panic!("Error: FromStr to Loc conversion"),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Op::Any => "any",
            Op::In => "in",
            Op::Reg => "reg",
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::Not => "not",
            Op::And => "and",
            Op::Nand => "nand",
            Op::Or => "or",
            Op::Nor => "nor",
            Op::Xor => "xor",
            Op::Xnor => "xnor",
            Op::Mux => "mux",
            Op::Equal => "eq",
            Op::Nequal => "neq",
            Op::Gt => "gt",
            Op::Lt => "lt",
            Op::Ge => "ge",
            Op::Le => "le",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Loc::Any => "any",
            Loc::Hole => "??",
            Loc::Lut => "lut",
            Loc::Lum => "lum",
            Loc::Dsp => "dsp",
            Loc::Ram => "ram",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.op, self.ty, self.loc)
    }
}
