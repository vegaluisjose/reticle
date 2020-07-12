use crate::lang::ast::{DataType, Expr, PlacedOp, Prog};
use petgraph::dot::{Config, Dot};
use petgraph::graph;
use petgraph::prelude::Graph;
use std::collections::HashMap;

pub type Ty = DataType;

#[derive(Clone, Debug)]
pub enum Op {
    Any,
    Ref,
    Reg,
    Add,
    Sub,
    Mul,
}

impl Op {
    pub fn from_expr(input: &Expr) -> Op {
        match input {
            Expr::Ref(_) => Op::Ref,
            _ => panic!("Error: only reference conversion supported"),
        }
    }

    pub fn from_placed_op(input: &PlacedOp) -> Op {
        match input {
            PlacedOp::Reg => Op::Reg,
            PlacedOp::Add => Op::Add,
            PlacedOp::Sub => Op::Sub,
            PlacedOp::Mul => Op::Mul,
            _ => panic!("Error: op not supported"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instr {
    op: Op,
    ty: Ty,
}

impl Instr {
    pub fn new(op: Op, ty: Ty) -> Instr {
        Instr { op: op, ty: ty }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    instr: Instr,
}

impl Node {
    pub fn new(name: &str, instr: Instr) -> Node {
        Node {
            name: name.to_string(),
            instr: instr,
        }
    }
}

// create edge type, so we can output better dot graphs
#[derive(Clone, Debug)]
pub struct Edge;

impl Edge {
    pub fn new() -> Edge {
        Edge {}
    }
}

#[derive(Clone, Debug)]
pub struct Pattern {
    name: String,
    seq: Vec<Instr>,
    cost: i32,
}

type Dag = Graph<Node, Edge>;
type DagIx = graph::NodeIndex;

pub struct DAG {
    dag: Dag,
    env: HashMap<String, DagIx>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            dag: Dag::new(),
            env: HashMap::new(),
        }
    }

    fn create_node_from_expr(&mut self, input: &Expr, ty: &Ty) {
        let op = Op::from_expr(input);
        let instr = Instr::new(op, ty.clone());
        let ix = self.dag.add_node(Node::new(&input.id(), instr));
        self.env.insert(input.id(), ix);
    }

    fn create_node_from_placed_op(&mut self, id: &str, input: &PlacedOp, ty: &Ty) {
        let op = Op::from_placed_op(input);
        let instr = Instr::new(op, ty.clone());
        let ix = self.dag.add_node(Node::new(id, instr));
        self.env.insert(id.to_string(), ix);
    }

    fn create_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.env.get(from) {
            if let Some(to_ix) = self.env.get(to) {
                if let None = self.dag.find_edge(*from_ix, *to_ix) {
                    self.dag.add_edge(*from_ix, *to_ix, Edge::new());
                }
            }
        }
    }

    pub fn from_prog(&mut self, input: &Prog) {
        assert!(input.defs.len() == 1, "Error: single component prog atm");
        for def in input.defs.iter() {
            for decl in def.body().iter() {
                assert!(decl.outputs().len() == 1, "Error: single output atm");
                let params = decl.params();
                let lhs = &params[0];
                let rhs = &params[1];
                let ty = decl.outputs()[0].datatype();
                let dst = decl.outputs()[0].clone();
                if !self.env.contains_key(&lhs.id()) {
                    self.create_node_from_expr(&lhs, &ty);
                }
                if !self.env.contains_key(&rhs.id()) {
                    self.create_node_from_expr(&rhs, &ty);
                }
                if !self.env.contains_key(&dst.id()) {
                    self.create_node_from_placed_op(&dst.id(), decl.placed_op(), &ty);
                }
                self.create_edge(&dst.id(), &lhs.id());
                self.create_edge(&dst.id(), &rhs.id());
            }
        }
        println!("{:?}", Dot::with_config(&self.dag, &[Config::EdgeNoLabel]));
    }
}
