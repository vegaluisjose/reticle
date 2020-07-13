use crate::passes::select::dag::*;
use crate::passes::select::instr::*;
use std::fmt;
use petgraph::dot::{Config, Dot};

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
            InstrLoc::Any => "any".to_string(),
            InstrLoc::Unknown => "??".to_string(),
            InstrLoc::Lut => "lut".to_string(),
            InstrLoc::Lum => "lum".to_string(),
            InstrLoc::Dsp => "dsp".to_string(),
            InstrLoc::Ram => "ram".to_string(),
            InstrLoc::Ref(n) => format!("loc({})", n),
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

impl fmt::Display for DAG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Dot::with_config(&self.dag, &[Config::EdgeNoLabel]))
    }
}