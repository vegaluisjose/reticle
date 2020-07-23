use crate::passes::select::instr::*;
use std::str::FromStr;

impl FromStr for Op {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
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
        match input {
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
