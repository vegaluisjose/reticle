use crate::passes::select::instr::*;
use std::fmt;

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
