use crate::backend::target::descriptor::{Descriptor, Tile};
use crate::passes::select::instr::*;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use petgraph::visit::{Dfs, DfsPostOrder};
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

    fn is_match(&self, start: SDNodeIx, pattern: &Pattern) -> bool {
        let mut is_match: bool = true;
        let mut pat_instr = pattern.instr.iter();
        let mut visit = Dfs::new(&self.graph, start);
        while let Some(ix) = visit.next(&self.graph) {
            if let Some(instr) = pat_instr.next() {
                if let Some(node) = self.graph.node_weight(ix) {
                    if instr.op != Op::Any {
                        if instr.op != node.instr.op {
                            is_match = false;
                            break;
                        }
                    }
                    if instr.ty != node.instr.ty {
                        is_match = false;
                        break;
                    }
                    if node.instr.loc != Loc::Hole {
                        if instr.loc != Loc::Any && instr.loc != node.instr.loc {
                            is_match = false;
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }
        is_match && pat_instr.len() == 0
    }

    fn estimate_cost(&self, ix: SDNodeIx) -> f32 {
        if let Some(node) = self.graph.node_weight(ix) {
            if node.instr.op == Op::In {
                0 as f32
            } else {
                if let Some(tile) = &node.tile {
                    tile.pattern.cost as f32
                } else {
                    f32::INFINITY
                }
            }
        } else {
            panic!("Error: node index does not seems to exists")
        }
    }

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

    pub fn select(&mut self, name: &str, descriptor: &Descriptor) {
        if self.contains_sdnode(name) {
            for tile in descriptor.tiles.iter() {
                if let Some(start) = self.ctx.get(name) {
                    let mut dag_iter = DfsPostOrder::new(&self.graph, *start);
                    while let Some(ix) = dag_iter.next(&self.graph) {
                        if self.is_match(ix, &tile.pattern) {
                            let cost = tile.pattern.cost.clone() as f32;
                            if cost < self.estimate_cost(ix) {
                                println!("there is match");
                            }
                        }
                    }
                }
            }
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
