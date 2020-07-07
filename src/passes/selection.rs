use petgraph::dot::{Config, Dot};
use petgraph::graph;
use petgraph::prelude::Graph;
use petgraph::visit::{DfsPostOrder, Dfs};

#[derive(Clone, Debug)]
pub enum Op {
    Ref,
    Add,
    Mul,
    Reg,
    Any,
}

impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op::Any, _) => true,
            (_, Op::Any) => true,
            (Op::Ref, Op::Ref) => true,
            (Op::Add, Op::Add) => true,
            (Op::Mul, Op::Mul) => true,
            (Op::Reg, Op::Reg) => true,
            (_, _) => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Loc {
    Gen,
    Lut,
    Dsp,
    Any,
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Loc::Any, _) => true,
            (_, Loc::Any) => true,
            (Loc::Gen, Loc::Gen) => true,
            (Loc::Lut, Loc::Lut) => true,
            (Loc::Dsp, Loc::Dsp) => true,
            (_, _) => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlacedOp {
    op: Op,
    loc: Loc,
}

impl PlacedOp {
    pub fn new(op: Op, loc: Loc) -> PlacedOp {
        PlacedOp { op: op, loc: loc }
    }

    pub fn new_gen_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Gen,
        }
    }

    pub fn new_lut_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Lut,
        }
    }

    pub fn new_dsp_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Dsp,
        }
    }

    pub fn new_any_op(op: Op) -> PlacedOp {
        PlacedOp {
            op: op,
            loc: Loc::Any,
        }
    }

    pub fn cost(&self) -> i128 {
        match (&self.op, &self.loc) {
            (Op::Add, Loc::Gen) => 9,
            (Op::Add, Loc::Lut) => 8,
            (Op::Add, Loc::Dsp) => 2,
            (Op::Mul, Loc::Gen) => 9,
            (Op::Mul, Loc::Lut) => 8,
            (Op::Mul, Loc::Dsp) => 2,
            (Op::Reg, Loc::Lut) => -4,
            (Op::Reg, Loc::Dsp) => -1,
            (_, _) => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    placed_op: PlacedOp,
}

impl Node {
    pub fn new(name: &str, placed_op: PlacedOp) -> Node {
        Node {
            name: name.to_string(),
            placed_op: placed_op,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pattern {
    name: String,
    cost: i128,
    ops: Vec<PlacedOp>,
}

impl Pattern {
    pub fn new(name: &str, cost: i128) -> Pattern {
        Pattern {
            name: name.to_string(),
            cost: cost,
            ops: Vec::new(),
        }
    }

    pub fn push_op(&mut self, op: PlacedOp) {
        self.ops.push(op);
    }
}

fn pat_dsp_muladd() -> Pattern {
    let mut pat = Pattern::new("dsp_muladd", 1);
    pat.push_op(PlacedOp::new_dsp_op(Op::Add));
    pat.push_op(PlacedOp::new_dsp_op(Op::Mul));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat
}

fn pat_dsp_mul() -> Pattern {
    let mut pat = Pattern::new("dsp_mul", 4);
    pat.push_op(PlacedOp::new_dsp_op(Op::Mul));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat
}

fn pat_dsp_add() -> Pattern {
    let mut pat = Pattern::new("dsp_add", 4);
    pat.push_op(PlacedOp::new_dsp_op(Op::Add));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat.push_op(PlacedOp::new_dsp_op(Op::Any));
    pat
}

pub type DAG = Graph<Node, ()>;
pub type DAGIx = graph::NodeIndex;

fn find_matches(graph: &DAG, ix: DAGIx, patterns: &Vec<Pattern>) {
    let mut root = DfsPostOrder::new(graph, ix);
    while let Some(nix) = root.next(graph) {
        let mut pat_iter = patterns.iter();
        while let Some(pat) = pat_iter.next() {
            let mut node_cost: i128 = 0;
            let mut ops = pat.ops.iter();
            // check if there is a pattern match
            let mut is_match: bool = true;
            let mut subgraph = Dfs::new(graph, nix);
            while let Some(six) = subgraph.next(graph) {
                if let Some(placed_op) = ops.next() {
                    if let Some(node) = graph.node_weight(six) {
                        node_cost += node.placed_op.cost();
                        if node.placed_op.op != placed_op.op {
                            is_match = false;
                        }
                    }
                } else {
                    break;
                }
            }
            if is_match && ops.len() == 0 {
                // check all nodes in the pattern
                if let Some(node) = graph.node_weight(nix) {
                    println!(
                        "node-name:{} node-cost:{} pat-name:{} pat-cost:{}",
                        node.name, node_cost, pat.name, pat.cost
                    );
                }
            }
        }
    }
}

pub fn main() {
    let mut graph = DAG::new();
    let a = graph.add_node(Node::new("a", PlacedOp::new_gen_op(Op::Ref)));
    let b = graph.add_node(Node::new("b", PlacedOp::new_gen_op(Op::Ref)));
    let c = graph.add_node(Node::new("c", PlacedOp::new_gen_op(Op::Ref)));
    let t0 = graph.add_node(Node::new("t0", PlacedOp::new_gen_op(Op::Mul)));
    let t1 = graph.add_node(Node::new("t1", PlacedOp::new_gen_op(Op::Add)));

    graph.add_edge(t0, a, ());
    graph.add_edge(t0, b, ());
    graph.add_edge(t1, t0, ());
    graph.add_edge(t1, c, ());

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let patterns = vec![pat_dsp_add(), pat_dsp_mul(), pat_dsp_muladd()];

    find_matches(&graph, t1, &patterns);
}
