use crate::lang::ast::{Instr, Port};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;

pub type DfgId = String;
pub type DfgIx = NodeIndex;
pub type DfgGraph = Graph<DfgNode, DfgEdge>;
pub type DfgCtx = HashMap<DfgId, DfgIx>;

#[derive(Clone, Debug)]
pub enum DfgNodeValue {
    Inp(Port),
    Out(Port),
    Ins(Instr),
}

#[derive(Clone, Debug)]
pub struct DfgNode {
    pub value: DfgNodeValue,
    pub visited: bool,
    pub root: bool,
}

#[derive(Default, Clone, Debug)]
pub struct DfgEdge;

#[derive(Clone, Debug)]
pub struct Dfg {
    pub graph: DfgGraph,
    pub ctx: DfgCtx,
    pub roots: DfgCtx,
}
