use crate::lang::ast::{Instr, Port, Prog};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum DagNodeValue {
    Inp(Port),
    Out(Port),
    Ins(Instr),
}

#[derive(Clone, Debug)]
pub struct DagNode {
    pub value: DagNodeValue,
    pub visited: bool,
    pub root: bool,
}

#[derive(Default, Clone, Debug)]
pub struct DagEdge;

pub type DGraph = Graph<DagNode, DagEdge>;
pub type DagIx = NodeIndex;
pub type DagId = String;
pub type DagCtx = HashMap<DagId, DagIx>;

#[derive(Clone, Debug)]
pub struct Dag {
    pub dag: DGraph,
    pub ctx: DagCtx,
}

impl From<Port> for DagNodeValue {
    fn from(port: Port) -> Self {
        match port {
            Port::Input { .. } => DagNodeValue::Inp(port),
            Port::Output { .. } => DagNodeValue::Out(port),
        }
    }
}

impl From<Instr> for DagNodeValue {
    fn from(instr: Instr) -> Self {
        DagNodeValue::Ins(instr)
    }
}

impl DagNodeValue {
    pub fn id(&self) -> String {
        match self {
            DagNodeValue::Inp(port) => port.id(),
            DagNodeValue::Out(port) => port.id(),
            DagNodeValue::Ins(instr) => instr.id(),
        }
    }

    pub fn op(&self) -> String {
        match self {
            DagNodeValue::Inp(_) => "input".to_string(),
            DagNodeValue::Out(_) => "output".to_string(),
            DagNodeValue::Ins(instr) => {
                if instr.is_std() {
                    instr.std_op().to_string()
                } else {
                    instr.prim_op().to_string()
                }
            }
        }
    }
}

impl DagNode {
    pub fn new(value: DagNodeValue) -> DagNode {
        DagNode {
            value,
            visited: false,
            root: false,
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn set_visited(&mut self) {
        self.visited = true;
    }

    pub fn set_root(&mut self) {
        self.root = true;
    }
}

impl Default for Dag {
    fn default() -> Dag {
        Dag {
            dag: DGraph::new(),
            ctx: DagCtx::new(),
        }
    }
}

impl Dag {
    pub fn add_node(&mut self, name: &str, value: DagNodeValue) {
        let ix = self.dag.add_node(DagNode::new(value));
        self.ctx.insert(name.to_string(), ix);
    }

    pub fn contains_node(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.ctx.get(from) {
            if let Some(to_ix) = self.ctx.get(to) {
                if self.dag.find_edge(*from_ix, *to_ix).is_none() {
                    self.dag.add_edge(*from_ix, *to_ix, DagEdge::default());
                }
            }
        }
    }
}

impl fmt::Display for DagNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.value.id(), self.value.op())
    }
}

impl fmt::Display for DagEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Dag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Dot::with_config(&self.dag, &[Config::EdgeNoLabel]))
    }
}

impl From<Prog> for Dag {
    fn from(prog: Prog) -> Self {
        let mut dag = Dag::default();
        if let Some(def) = prog.defs().iter().next() {
            for input in def.inputs().iter() {
                if !dag.contains_node(&input.id()) {
                    let val = DagNodeValue::from(input.clone());
                    dag.add_node(&input.id(), val);
                }
            }
            for output in def.outputs().iter() {
                if !dag.contains_node(&output.id()) {
                    let val = DagNodeValue::from(output.clone());
                    dag.add_node(&output.id(), val);
                }
            }
            for instr in def.body().iter() {
                if !dag.contains_node(&instr.id()) {
                    let val = DagNodeValue::from(instr.clone());
                    dag.add_node(&instr.id(), val);
                }
            }
            for instr in def.body().iter() {
                for param in instr.params().iter() {
                    dag.add_edge(&param.id(), &instr.id());
                }
            }
        }
        dag
    }
}

pub fn example(prog: &Prog) {
    let dag = Dag::from(prog.clone());
    println!("{}", dag);
    println!("{}", prog);
}
