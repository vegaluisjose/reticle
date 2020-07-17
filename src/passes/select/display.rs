use crate::passes::select::dag::*;
use crate::passes::select::dag_instr::*;
use petgraph::dot::{Config, Dot};
use std::fmt;

impl fmt::Display for DagOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DagOp::Any => "any",
            DagOp::Ref => "ref",
            DagOp::Inp => "inp",
            DagOp::Reg => "reg",
            DagOp::Add => "add",
            DagOp::Sub => "sub",
            DagOp::Mul => "mul",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for DagLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DagLoc::Any => "any".to_string(),
            DagLoc::Unknown => "??".to_string(),
            DagLoc::Lut => "lut".to_string(),
            DagLoc::Lum => "lum".to_string(),
            DagLoc::Dsp => "dsp".to_string(),
            DagLoc::Ram => "ram".to_string(),
            DagLoc::Ref(n) => format!("loc({})", n),
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for DagInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.op, self.ty, self.loc)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.instr)
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
