use crate::passes::select::dag::*;
use crate::passes::select::instr::*;
use std::fmt;

impl fmt::Display for InstrOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            InstrOp::Any => "any",
            InstrOp::Ref => "ref",
            InstrOp::Reg => "reg",
            InstrOp::Add => "add",
            InstrOp::Sub => "sub",
            InstrOp::Mul => "mul",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for InstrLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            InstrLoc::Any => "any",
            InstrLoc::Unknown => "??",
            InstrLoc::Lut => "lut",
            InstrLoc::Lum => "lum",
            InstrLoc::Dsp => "dsp",
            InstrLoc::Ram => "ram",
            InstrLoc::Ref(n) => n,
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.instr)
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<{}> @{}", self.op, self.ty, self.loc)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
