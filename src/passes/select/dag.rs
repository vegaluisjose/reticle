use crate::lang::ast::{DataType, Instr, Def, Expr, Loc, PlacedOp, Port, Prog};
use crate::passes::select::cost::*;
use crate::passes::select::instr::*;
use crate::passes::select::pattern::*;
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use petgraph::visit::{Dfs, DfsPostOrder};
use petgraph::Direction;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub instr: DagInstr,
}

impl Node {
    pub fn new(name: &str, instr: DagInstr) -> Node {
        Node {
            name: name.to_string(),
            instr: instr,
        }
    }

    pub fn to_ast_instr(&self, params: &Vec<String>) -> Instr {
        Instr::Placed {
            id: self.name.to_string(),
            op: self.instr.op.to_placed_op(),
            ty: self.instr.ty.clone(),
            attrs: vec![],
            params: vec![
                Expr::Ref(params[0].to_string()),
                Expr::Ref(params[1].to_string()),
            ],
            loc: self.instr.loc.to_loc(),
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
pub struct Signature {
    inputs: Vec<Port>,
    outputs: Vec<Port>,
}

impl Signature {
    pub fn new(inputs: &Vec<Port>, outputs: &Vec<Port>) -> Signature {
        Signature {
            inputs: inputs.to_vec(),
            outputs: outputs.to_vec(),
        }
    }
}

type Dag = Graph<Node, Edge>;
type DagIx = NodeIndex;

pub struct DAG {
    pub dag: Dag,
    pub index_map: HashMap<String, DagIx>,
    pub signature_map: HashMap<String, Signature>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            dag: Dag::new(),
            index_map: HashMap::new(),
            signature_map: HashMap::new(),
        }
    }

    fn create_node_from_expr(&mut self, input: &Expr, ty: &DataType) {
        let instr_op = DagOp::from_expr(input);
        let instr = DagInstr::new(instr_op, ty.clone(), DagLoc::Lut);
        let ix = self.dag.add_node(Node::new(&input.id(), instr));
        self.index_map.insert(input.id(), ix);
    }

    fn create_node_from_placed_op(&mut self, id: &str, input: &PlacedOp, ty: &DataType, loc: &Loc) {
        let instr_op = DagOp::from_placed_op(input);
        let instr_loc = DagLoc::from_loc(loc);
        let instr = DagInstr::new(instr_op, ty.clone(), instr_loc);
        let ix = self.dag.add_node(Node::new(id, instr));
        self.index_map.insert(id.to_string(), ix);
    }

    fn create_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.index_map.get(from) {
            if let Some(to_ix) = self.index_map.get(to) {
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
                    if instr.op != DagOp::Any {
                        if instr.op != node.instr.op {
                            is_match = false;
                        }
                    } else {
                        if instr.loc != node.instr.loc {
                            is_match = false;
                        }
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
                    if instr.op != DagOp::Any {
                        if node.name == node_id {
                            node.instr.loc = instr.loc.clone();
                        } else {
                            node.instr.loc = DagLoc::Ref(node_id.to_string())
                        }
                    }
                }
            }
        }
    }

    pub fn from_prog(&mut self, input: &Prog) {
        assert!(input.defs.len() == 1, "Error: single component prog atm");
        for def in input.defs.iter() {
            if !self.signature_map.contains_key(&def.name()) {
                let sig = Signature::new(def.inputs(), def.outputs());
                self.signature_map.insert(def.name(), sig);
            } else {
                panic!("Error: duplicate component definition");
            }
            for instr in def.body().iter() {
                let params = instr.params();
                let lhs = &params[0];
                let rhs = &params[1];
                let loc = instr.loc();
                if !self.index_map.contains_key(&lhs.id()) {
                    self.create_node_from_expr(&lhs, &instr.ty());
                }
                if !self.index_map.contains_key(&rhs.id()) {
                    self.create_node_from_expr(&rhs, &instr.ty());
                }
                if !self.index_map.contains_key(&instr.id()) {
                    self.create_node_from_placed_op(&instr.id(), instr.placed_op(), &instr.ty(), &loc);
                }
                self.create_edge(&instr.id(), &lhs.id());
                self.create_edge(&instr.id(), &rhs.id());
            }
        }
    }

    pub fn select(&mut self) {
        let sig_map = self.signature_map.clone(); // copy here, and make borrow-checker happy
        for (_, sig) in sig_map.iter() {
            for output in sig.outputs.iter() {
                for pat in patterns().iter() {
                    if let Some(rix) = self.index_map.get(&output.id()) {
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
    }

    pub fn to_prog(&self) -> Prog {
        let mut prog = Prog::new();
        for (def_name, sig) in self.signature_map.iter() {
            let mut def = Def::new_with_ports(def_name, &sig.inputs, &sig.outputs);
            for output in sig.outputs.iter() {
                if let Some(oix) = self.index_map.get(&output.id()) {
                    let mut dag_iter = DfsPostOrder::new(&self.dag, *oix);
                    while let Some(ix) = dag_iter.next(&self.dag) {
                        if let Some(node) = self.dag.node_weight(ix) {
                            if node.instr.op != DagOp::Ref {
                                let mut children =
                                    self.dag.neighbors_directed(ix, Direction::Outgoing);
                                let mut params: Vec<String> = Vec::new();
                                while let Some(cix) = children.next() {
                                    if let Some(children_node) = self.dag.node_weight(cix) {
                                        params.push(children_node.name.to_string());
                                    }
                                }
                                def.add_instr(node.to_ast_instr(&params));
                            }
                        }
                    }
                }
            }
            prog.add_def(def);
        }
        prog
    }
}
