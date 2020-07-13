use crate::lang::ast::{Expr, Loc, PlacedOp, Decl, Port, Op, Prog};
use crate::passes::select::cost::*;
use crate::passes::select::instr::*;
use crate::passes::select::pattern::*;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use petgraph::prelude::Graph;
use petgraph::visit::{Dfs, DfsPostOrder};
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

    pub fn to_ast_instr(&self, params: &Vec<String>) -> Decl {
        let placed_op = Op::Placed {
            op: self.instr.op.to_placed_op(),
            attrs: vec![],
            params: vec![Expr::Ref(params[0].to_string()), Expr::Ref(params[1].to_string())],
            loc: self.instr.loc.to_loc(),
        };
        let output = Port::Output {
            id: self.name.to_string(),
            datatype: self.instr.ty.clone(),
        };
        Decl::Instr {
            op: placed_op,
            outputs: vec![output],
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
type DagIx = NodeIndex;

pub struct DAG {
    pub dag: Dag,
    pub nodes: HashMap<String, DagIx>,
    pub roots: Vec<String>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            dag: Dag::new(),
            nodes: HashMap::new(),
            roots: Vec::new(),
        }
    }

    fn create_node_from_expr(&mut self, input: &Expr, ty: &InstrTy) {
        let instr_op = InstrOp::from_expr(input);
        let instr = Instr::new(instr_op, ty.clone(), InstrLoc::Any);
        let ix = self.dag.add_node(Node::new(&input.id(), instr));
        self.nodes.insert(input.id(), ix);
    }

    fn create_node_from_placed_op(&mut self, id: &str, input: &PlacedOp, ty: &InstrTy, loc: &Loc) {
        let instr_op = InstrOp::from_placed_op(input);
        let instr_loc = InstrLoc::from_loc(loc);
        let instr = Instr::new(instr_op, ty.clone(), instr_loc);
        let ix = self.dag.add_node(Node::new(id, instr));
        self.nodes.insert(id.to_string(), ix);
    }

    fn create_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.nodes.get(from) {
            if let Some(to_ix) = self.nodes.get(to) {
                if let None = self.dag.find_edge(*from_ix, *to_ix) {
                    self.dag.add_edge(*from_ix, *to_ix, Edge::new());
                }
            }
        }
    }

    fn is_match(&self, start: DagIx, pattern: &Pattern) -> bool {
        let mut is_match: bool = true;
        let mut pat_instr = pattern.instr.iter();
        let mut visit = Dfs::new(&self.dag, start);
        while let Some(ix) = visit.next(&self.dag) {
            if let Some(instr) = pat_instr.next() {
                if let Some(node) = self.dag.node_weight(ix) {
                    if instr.op != InstrOp::Any && instr.op != node.instr.op {
                        is_match = false;
                    }
                }
            } else {
                break;
            }
        }
        is_match && pat_instr.len() == 0
    }

    fn estimate_cost(&self, start: DagIx) -> i32 {
        let mut cost: i32 = 0;
        let mut visit = Dfs::new(&self.dag, start);
        while let Some(ix) = visit.next(&self.dag) {
            if let Some(node) = self.dag.node_weight(ix) {
                cost += estimate_instr_cost(&node.instr);
            }
        }
        cost
    }

    fn find_ref_node(&self, start: DagIx, pattern: &Pattern) -> String {
        let mut node_id: String = String::new();
        let mut pat_instr = pattern.instr.iter();
        let mut visit = Dfs::new(&self.dag, start);
        while let Some(ix) = visit.next(&self.dag) {
            if let Some(instr) = pat_instr.next() {
                if let Some(node) = self.dag.node_weight(ix) {
                    if instr.op == node.instr.op {
                        node_id = node.name.to_string();
                    }
                }
            } else {
                break;
            }
        }
        node_id
    }

    fn rewrite(&mut self, start: DagIx, pattern: &Pattern) {
        let mut pat_instr = pattern.instr.iter();
        let mut visit = Dfs::new(&self.dag, start);
        let node_id: String = self.find_ref_node(start, pattern);
        while let Some(ix) = visit.next(&self.dag) {
            if let Some(instr) = pat_instr.next() {
                if let Some(node) = self.dag.node_weight_mut(ix) {
                    if instr.op != InstrOp::Any {
                        if node.name == node_id {
                            node.instr.loc = instr.loc.clone();
                        } else {
                            node.instr.loc = InstrLoc::Ref(node_id.to_string())
                        }
                    }
                }
            }
        }
    }

    pub fn create_dag_from_prog(&mut self, input: &Prog) {
        assert!(input.defs.len() == 1, "Error: single component prog atm");
        for def in input.defs.iter() {
            for port in def.outputs().iter() {
                self.roots.push(port.id());
            }
            for decl in def.body().iter() {
                assert!(decl.outputs().len() == 1, "Error: single output decl support atm");
                let params = decl.params();
                let lhs = &params[0];
                let rhs = &params[1];
                let ty = decl.outputs()[0].datatype();
                let dst = decl.outputs()[0].clone();
                let loc = decl.loc();
                if !self.nodes.contains_key(&lhs.id()) {
                    self.create_node_from_expr(&lhs, &ty);
                }
                if !self.nodes.contains_key(&rhs.id()) {
                    self.create_node_from_expr(&rhs, &ty);
                }
                if !self.nodes.contains_key(&dst.id()) {
                    self.create_node_from_placed_op(&dst.id(), decl.placed_op(), &ty, &loc);
                }
                self.create_edge(&dst.id(), &lhs.id());
                self.create_edge(&dst.id(), &rhs.id());
            }
        }
    }

    pub fn select(&mut self) {
        let roots = self.roots.clone(); // copy here, so we can mutate dag
        for rid in roots.iter() {
            for pat in patterns().iter() {
                if let Some(rix) = self.nodes.get(rid) {
                    let mut dag_iter = DfsPostOrder::new(&self.dag, *rix);
                    while let Some(ix) = dag_iter.next(&self.dag) {
                        if self.is_match(ix, pat) {
                            let cost = self.estimate_cost(ix);
                            if pat.cost < cost {
                                self.rewrite(ix, pat);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn to_prog(&self) {
        for rid in self.roots.iter() {
            if let Some(rix) = self.nodes.get(rid) {
                let mut dag_iter = DfsPostOrder::new(&self.dag, *rix);
                while let Some(ix) = dag_iter.next(&self.dag) {
                    if let Some(node) = self.dag.node_weight(ix) {
                        if node.instr.op != InstrOp::Ref {
                            let mut children = self.dag.neighbors_directed(ix, Direction::Outgoing);
                            let mut params: Vec<String> = Vec::new();
                            while let Some(cix) = children.next() {
                                if let Some(children_node) = self.dag.node_weight(cix) {
                                    params.push(children_node.name.to_string());
                                }
                            }
                            println!("{}", node.to_ast_instr(&params));
                        }
                    }
                }
            }
        }
    }
}
