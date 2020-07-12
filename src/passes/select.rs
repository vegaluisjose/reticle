use crate::lang::ast::{DataType, Expr, PlacedOp, Prog, Loc, Id};
use petgraph::dot::{Config, Dot};
use petgraph::graph;
use petgraph::prelude::Graph;
use std::collections::HashMap;
use std::fmt;

pub type InstrTy = DataType;

#[derive(Clone, Debug)]
pub enum InstrOp {
    Any,
    Ref,
    Reg,
    Add,
    Sub,
    Mul,
}

impl fmt::Display for InstrOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            InstrOp::Any => "any",
            InstrOp::Ref => "ref",
            InstrOp::Reg => "reg",
            InstrOp::Add => "add",
            InstrOp::Sub => "sub",
            InstrOp::Mul => "mul",
        };
        write!(f, "{}", name)
    }
}

impl InstrOp {
    pub fn from_expr(input: &Expr) -> InstrOp {
        match input {
            Expr::Ref(_) => InstrOp::Ref,
            _ => panic!("Error: only reference conversion supported"),
        }
    }

    pub fn from_placed_op(input: &PlacedOp) -> InstrOp {
        match input {
            PlacedOp::Reg => InstrOp::Reg,
            PlacedOp::Add => InstrOp::Add,
            PlacedOp::Sub => InstrOp::Sub,
            PlacedOp::Mul => InstrOp::Mul,
            _ => panic!("Error: op not supported"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InstrLoc {
    Any,
    Unknown,
    Lut,
    Lum,
    Dsp,
    Ram,
    Ref(Id),
}

impl InstrLoc {
    pub fn from_loc(input: &Loc) -> InstrLoc {
        match input {
            Loc::Unknown => InstrLoc::Unknown,
            Loc::Lut => InstrLoc::Lut,
            Loc::Lum => InstrLoc::Lum,
            Loc::Dsp => InstrLoc::Dsp,
            Loc::Ram => InstrLoc::Ram,
            Loc::Ref(n) => InstrLoc::Ref(n.to_string()),
        }
    }
}

impl fmt::Display for InstrLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            InstrLoc::Any => "any",
            InstrLoc::Unknown => "??",
            InstrLoc::Lut => "lut",
            InstrLoc::Lum => "lum",
            InstrLoc::Dsp => "dsp",
            InstrLoc::Ram => "ram",
            InstrLoc::Ref(n) => n,
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug)]
pub struct Instr {
    op: InstrOp,
    ty: InstrTy,
    loc: InstrLoc,
}

impl Instr {
    pub fn new(op: InstrOp, ty: InstrTy, loc: InstrLoc) -> Instr {
        Instr { op: op, ty: ty, loc: loc }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<{}> @{}", self.op, self.ty, self.loc)
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    instr: Instr,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.instr)
    }
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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
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
