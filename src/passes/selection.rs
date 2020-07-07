use petgraph::dot::{Config, Dot};
use petgraph::graph;
use petgraph::prelude::Graph;
use petgraph::visit::{DfsPostOrder, Dfs};

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Any,
    Ref,
    Add,
    Mul,
    Reg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loc {
    Any,
    Gen,
    Lut,
    Dsp,
    Equal(String),
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

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }

    pub fn cost(&self) -> i32 {
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
    cost: i32,
    ops: Vec<PlacedOp>,
}

impl Pattern {
    pub fn new(name: &str, cost: i32) -> Pattern {
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

fn estimate_cost(dag: &DAG, start: DAGIx) -> i32 {
    let mut cost: i32 = 0;
    let mut visit = Dfs::new(dag, start);
    while let Some(ix) = visit.next(dag) {
        if let Some(node) = dag.node_weight(ix) {
            cost += node.placed_op.cost();
        }
    }
    cost
}

fn is_match(dag: &DAG, start: DAGIx, pattern: &Pattern) -> bool {
    let mut is_match: bool = true;
    let mut pattern_ops = pattern.ops.iter();
    let mut visit = Dfs::new(dag, start);
    while let Some(ix) = visit.next(dag) {
        if let Some(pattern_op) = pattern_ops.next() {
            if let Some(node) = dag.node_weight(ix) {
                if pattern_op.op != Op::Any && node.placed_op.op != pattern_op.op {
                    is_match = false;
                }
            }
        } else {
            break;
        }
    }
    is_match && pattern_ops.len() == 0
}

fn debug(dag: &DAG, start: DAGIx, cost: i32, pattern: &Pattern) {
    if let Some(node) = dag.node_weight(start) {
        println!("new candidate, pattern:{} pattern-cost:{} node:{} node-cost:{}",
            pattern.name, pattern.cost, node.name, cost);
    }
}

fn rewrite(dag: &mut DAG, start: DAGIx, pattern: &Pattern) {
    let mut is_first: bool = true;
    let node_id: String = dag.node_weight(start).unwrap().name.to_string();
    let mut pattern_ops = pattern.ops.iter();
    let mut visit = Dfs::new(&*dag, start);
    while let Some(ix) = visit.next(&*dag) {
        if let Some(pattern_placed_op) = pattern_ops.next() {
            if let Some(node) = dag.node_weight_mut(ix) {
                if pattern_placed_op.op != Op::Any {
                    if is_first {
                        node.placed_op.loc = pattern_placed_op.loc.clone();
                        is_first = false;
                    } else {
                        node.placed_op.loc = Loc::Equal(node_id.to_string());
                    }
                }
            }
        }
    }
}

fn select(dag: &mut DAG, start: DAGIx, pattern: &Pattern) {
    let mut root = DfsPostOrder::new(&*dag, start);
    while let Some(ix) = root.next(&*dag) {
        let pattern_match = is_match(&*dag, ix, pattern);
        if pattern_match {
            let cost = estimate_cost(&*dag, ix);
            if pattern.cost < cost {
                debug(&*dag, ix, cost, pattern);
                rewrite(dag, ix, pattern);
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

    let patterns = vec![pat_dsp_mul(), pat_dsp_add(), pat_dsp_muladd()];

    for p in patterns.iter() {
        select(&mut graph, t1, p);
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    }
}
