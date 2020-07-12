use crate::lang::ast::{Expr, Loc, PlacedOp, Prog};
use crate::passes::select::instr::*;
use petgraph::dot::{Config, Dot};
use petgraph::graph;
use petgraph::prelude::Graph;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub instr: Instr,
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

type Dag = Graph<Node, Edge>;
type DagIx = graph::NodeIndex;

pub struct DAG {
    pub dag: Dag,
    pub env: HashMap<String, DagIx>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            dag: Dag::new(),
            env: HashMap::new(),
        }
    }

    fn create_node_from_expr(&mut self, input: &Expr, ty: &InstrTy) {
        let instr_op = InstrOp::from_expr(input);
        let instr = Instr::new(instr_op, ty.clone(), InstrLoc::Any);
        let ix = self.dag.add_node(Node::new(&input.id(), instr));
        self.env.insert(input.id(), ix);
    }

    fn create_node_from_placed_op(&mut self, id: &str, input: &PlacedOp, ty: &InstrTy, loc: &Loc) {
        let instr_op = InstrOp::from_placed_op(input);
        let instr_loc = InstrLoc::from_loc(loc);
        let instr = Instr::new(instr_op, ty.clone(), instr_loc);
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
                let loc = decl.loc();
                if !self.env.contains_key(&lhs.id()) {
                    self.create_node_from_expr(&lhs, &ty);
                }
                if !self.env.contains_key(&rhs.id()) {
                    self.create_node_from_expr(&rhs, &ty);
                }
                if !self.env.contains_key(&dst.id()) {
                    self.create_node_from_placed_op(&dst.id(), decl.placed_op(), &ty, &loc);
                }
                self.create_edge(&dst.id(), &lhs.id());
                self.create_edge(&dst.id(), &rhs.id());
            }
        }
        println!("{}", Dot::with_config(&self.dag, &[Config::EdgeNoLabel]));
    }
}
