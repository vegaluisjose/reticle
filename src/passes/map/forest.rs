use crate::lang::ast::{Instr, PrimOp, Ty};
use crate::passes::map::dag::{Dag, DagIx};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;
use std::fmt;

pub type TreeId = String;
pub type TreeTy = Ty;
pub type TreeIx = NodeIndex;
pub type TreeGraph = Graph<TreeNode, TreeEdge>;
pub type TreeCtx = HashMap<TreeId, TreeIx>;

#[derive(Clone, Debug)]
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
}

pub type Forest = HashMap<TreeId, Tree>;

impl From<PrimOp> for TreeOp {
    fn from(primop: PrimOp) -> Self {
        TreeOp::Prim(primop)
    }
}

impl From<Instr> for TreeNode {
    fn from(instr: Instr) -> Self {
        match instr {
            Instr::Prim {
                id,
                ty,
                op,
                attrs: _,
                params: _,
                loc: _,
            } => TreeNode {
                id,
                ty,
                op: TreeOp::from(op),
            },
            _ => panic!("Error: tree nodes only support prim instr"),
        }
    }
}

impl From<Dag> for Forest {
    fn from(dag: Dag) -> Self {
        let mut dag = dag;
        let mut forest = Forest::new();
        for (id, root) in dag.roots().clone().iter() {
            let mut tree = Tree::default();
            let mut stack: Vec<DagIx> = Vec::new();
            stack.push(*root);
            while !stack.is_empty() {
                if let Some(src_ix) = stack.pop() {
                    if let Some(src_node) = dag.graph.node_weight_mut(src_ix) {
                        src_node.set_visited();
                    }
                    if let Some(src_node) = dag.graph.node_weight(src_ix) {
                        if !tree.contains_node(&src_node.id()) {
                            tree.add_node(&src_node.id(), TreeNode::from(src_node.instr().clone()));
                        }
                        let incoming = dag.get_incoming_nodes(src_ix);
                        for param in src_node.instr().params().iter() {
                            if let Some(dst_ix) = incoming.get(&param.id()) {
                                if let Some(dst_node) = dag.graph.node_weight(*dst_ix) {
                                    tree.add_node(
                                        &dst_node.id(),
                                        TreeNode::from(dst_node.instr().clone()),
                                    );
                                    tree.add_edge(&src_node.id(), &dst_node.id());
                                }
                            } else {
                                tree.add_node(
                                    &param.id(),
                                    TreeNode::new_input(&param.id(), param.ty().clone()),
                                );
                                tree.add_edge(&src_node.id(), &param.id());
                            }
                        }
                        let incoming_ix: Vec<DagIx> = incoming.values().cloned().collect();
                        stack.extend(incoming_ix);
                    }
                }
            }
            forest.insert(id.to_string(), tree);
        }
        forest
    }
}

impl TreeNode {
    pub fn new_input(id: &str, ty: TreeTy) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op: TreeOp::Input,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub graph: TreeGraph,
    pub ctx: TreeCtx,
}

impl Default for Tree {
    fn default() -> Tree {
        Tree {
            graph: TreeGraph::new(),
            ctx: TreeCtx::new(),
        }
    }
}

impl Tree {
    pub fn contains_node(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
    }

    pub fn add_node(&mut self, name: &str, node: TreeNode) {
        let ix = self.graph.add_node(node);
        self.ctx.insert(name.to_string(), ix);
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.ctx.get(from) {
            if let Some(to_ix) = self.ctx.get(to) {
                if self.graph.find_edge(*from_ix, *to_ix).is_none() {
                    self.graph.add_edge(*from_ix, *to_ix, TreeEdge::default());
                }
            }
        }
    }
}

impl fmt::Display for TreeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            TreeOp::Input => "in".to_string(),
            TreeOp::Prim(op) => op.to_string(),
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.id(), self.op)
    }
}

impl fmt::Display for TreeEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
}
