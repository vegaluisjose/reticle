use crate::backend::target::descriptor::Tile;
use crate::passes::select::instr::*;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct SDNode {
    pub name: String,
    pub instr: Instr,
    pub tile: Option<Tile>,
}

#[derive(Clone, Debug)]
pub struct SDEdge;

type SDGraph = Graph<SDNode, SDEdge>;
type SDNodeIx = NodeIndex;
type SDId = String;
type SDCtx = HashMap<SDId, SDNodeIx>;

#[derive(Clone, Debug)]
pub struct SDag {
    pub graph: SDGraph,
    pub ctx: SDCtx,
}

impl SDNode {
    pub fn new(name: &str, instr: Instr) -> SDNode {
        SDNode {
            name: name.to_string(),
            instr: instr,
            tile: None,
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
            ctx: SDCtx::new(),
        }
    }

    pub fn add_sdnode(&mut self, name: &str, instr: Instr) {
        let ix = self.graph.add_node(SDNode::new(name, instr));
        self.ctx.insert(name.to_string(), ix);
    }

    pub fn add_sdedge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.ctx.get(from) {
            if let Some(to_ix) = self.ctx.get(to) {
                if let None = self.graph.find_edge(*from_ix, *to_ix) {
                    self.graph.add_edge(*from_ix, *to_ix, SDEdge::new());
                }
            }
        }
    }

    pub fn contains_sdnode(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
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
