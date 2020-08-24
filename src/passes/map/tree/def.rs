use crate::backend::target::Tile;
use crate::lang::ast::{Loc, PrimOp, Ty};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;

pub type TreeId = String;
pub type TreeTy = Ty;
pub type TreeLoc = Loc;
pub type TreeIx = NodeIndex;
pub type TreeGraph = Graph<TreeNode, TreeEdge>;
pub type TreeCtx = HashMap<TreeId, TreeIx>;

#[derive(Clone, Debug, PartialEq)]
pub enum TreeOp {
    Input,
    Prim(PrimOp),
}

#[derive(Default, Clone, Debug)]
pub struct TreeEdge;

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: TreeId,
    pub ty: TreeTy,
    pub op: TreeOp,
    pub loc: TreeLoc,
    pub matched: bool,
    pub tile: Option<Tile>,
    pub cost: f32,
}

#[derive(Default, Clone, Debug)]
pub struct Tree {
    pub root_index: Option<TreeIx>,
    pub graph: TreeGraph,
    pub ctx: TreeCtx,
}
