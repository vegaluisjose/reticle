use petgraph::dot::{Config, Dot};
use petgraph::prelude::Graph;
use petgraph::visit::DfsPostOrder;

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

#[derive(Clone, Debug, PartialEq)]
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
    ops: Vec<PlacedOp>,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern { ops: Vec::new() }
    }

    pub fn push_op(&mut self, op: PlacedOp) {
        self.ops.push(op);
    }
}

fn pat_0() -> Pattern {
    let mut pat = Pattern::new();
    pat.push_op(PlacedOp::new_gen_op(Op::Any));
    pat.push_op(PlacedOp::new_gen_op(Op::Any));
    pat.push_op(PlacedOp::new_gen_op(Op::Mul));
    pat.push_op(PlacedOp::new_gen_op(Op::Any));
    pat.push_op(PlacedOp::new_gen_op(Op::Add));
    pat
}

pub fn main() {
    let mut graph = Graph::<Node, ()>::new();
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

    let mut root = DfsPostOrder::new(&graph, t1);
    while let Some(idx) = root.next(&graph) {
        let p0 = pat_0();
        let mut pat_ops = p0.ops.iter();
        // check if there is a pattern match
        let mut pat_match: bool = true;
        let mut subgraph = DfsPostOrder::new(&graph, idx);
        while let Some(pat_op) = pat_ops.next() {
            if let Some(sub_idx) = subgraph.next(&graph) {
                if let Some(node) = graph.node_weight(sub_idx) {
                    if node.placed_op != *pat_op {
                        pat_match = false;
                    }
                }
            } else {
                pat_match = false;
                break;
            }
        }
        if pat_match && pat_ops.len() == 0 {
            // check all nodes in the pattern
            if let Some(node) = graph.node_weight(idx) {
                println!("This node is a candidate: {:?}", node);
            }
        }
    }
}
