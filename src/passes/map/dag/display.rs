use crate::passes::map::dag::{Dag, DagEdge, DagNode};
use petgraph::dot::{Config, Dot};
use std::fmt;

impl fmt::Display for DagNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{} op:{}", self.value.id(), self.value.op_name())
    }
}

impl fmt::Display for DagEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Dag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
}
