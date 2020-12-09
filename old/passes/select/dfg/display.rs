use crate::passes::select::dfg::{Dfg, DfgEdge, DfgNode};
use petgraph::dot::{Config, Dot};
use std::fmt;

impl fmt::Display for DfgNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{} op:{}", self.value.id(), self.value.op_name())
    }
}

impl fmt::Display for DfgEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Dfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
}
