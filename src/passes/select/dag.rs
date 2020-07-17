use crate::passes::select::instr::*;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::fmt;

#[derive(Clone, Debug)]
pub struct SDNode {
    pub name: String,
    pub instr: Instr,
}

#[derive(Clone, Debug)]
pub struct SDEdge;

type SDGraph = Graph<SDNode, SDEdge>;
type SDNodeIx = NodeIndex;

#[derive(Clone, Debug)]
pub struct SDag {
    pub graph: SDGraph,
}

impl SDNode {
    pub fn new(name: &str, instr: Instr) -> SDNode {
        SDNode {
            name: name.to_string(),
            instr: instr,
        }
    }
}

impl SDEdge {
    pub fn new() -> SDEdge {
        SDEdge {}
    }
}

impl SDag {
    pub fn new() -> SDag {
        SDag {
            graph: SDGraph::new(),
        }
    }
}

impl fmt::Display for SDNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.instr)
    }
}

impl fmt::Display for SDEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for SDag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
}
