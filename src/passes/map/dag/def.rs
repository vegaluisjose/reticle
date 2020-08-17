use crate::lang::ast::{Instr, Port};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;

pub type DagId = String;
pub type DagIx = NodeIndex;
pub type DagGraph = Graph<DagNode, DagEdge>;
pub type DagCtx = HashMap<DagId, DagIx>;

#[derive(Clone, Debug)]
pub enum DagNodeValue {
    Inp(Port),
    Out(Port),
    Ins(Instr),
}

#[derive(Clone, Debug)]
pub struct DagNode {
    pub value: DagNodeValue,
    pub visited: bool,
    pub root: bool,
}

#[derive(Default, Clone, Debug)]
pub struct DagEdge;

#[derive(Clone, Debug)]
pub struct Dag {
    pub graph: DagGraph,
    pub ctx: DagCtx,
    pub roots: DagCtx,
}
