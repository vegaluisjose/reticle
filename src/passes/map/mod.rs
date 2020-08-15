use crate::lang::ast::{Instr, Port, Prog};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use petgraph::visit::Dfs;
use petgraph::Direction;
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
}

#[derive(Default, Clone, Debug)]
pub struct DagEdge;

pub type DGraph = Graph<DagNode, DagEdge>;
pub type DagIx = NodeIndex;
pub type DagId = String;
pub type DagCtx = HashMap<DagId, DagIx>;

#[derive(Clone, Debug)]
pub struct Dag {
    pub graph: DGraph,
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

    pub fn is_std_instr(&self) -> bool {
        match self {
            DagNodeValue::Ins(instr) => instr.is_std(),
            _ => false,
        }
    }

    pub fn is_prim_instr(&self) -> bool {
        match self {
            DagNodeValue::Ins(instr) => instr.is_prim(),
            _ => false,
        }
    }
}

impl DagNode {
    pub fn new(value: DagNodeValue) -> DagNode {
        DagNode { value }
    }
}

impl Default for Dag {
    fn default() -> Dag {
        Dag {
            graph: DGraph::new(),
            ctx: DagCtx::new(),
        }
    }
}

impl Dag {
    pub fn add_node(&mut self, name: &str, value: DagNodeValue) {
        let ix = self.graph.add_node(DagNode::new(value));
        self.ctx.insert(name.to_string(), ix);
    }

    pub fn contains_node(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
    }

    pub fn get_node_index(&self, name: &str) -> Option<&DagIx> {
        self.ctx.get(name)
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.ctx.get(from) {
            if let Some(to_ix) = self.ctx.get(to) {
                if self.graph.find_edge(*from_ix, *to_ix).is_none() {
                    self.graph.add_edge(*from_ix, *to_ix, DagEdge::default());
                }
            }
        }
    }
}

impl fmt::Display for DagNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{} op:{}", self.value.id(), self.value.op())
    }
}

impl fmt::Display for DagEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Dag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
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
    let mut root = DagCtx::new();
    if let Some(def) = prog.defs().iter().next() {
        for input in def.inputs().iter() {
            if let Some(ix) = dag.get_node_index(&input.id()) {
                let mut visit = Dfs::new(&dag.graph, *ix);
                while let Some(next) = visit.next(&dag.graph) {
                    if let Some(node) = dag.graph.node_weight(next) {
                        let neighbors = dag.graph.neighbors_directed(next, Direction::Outgoing);
                        let n = neighbors.count();
                        if node.value.is_prim_instr()
                            && n != 1
                            && !root.contains_key(&node.value.id())
                        {
                            root.insert(node.value.id(), next);
                        }
                    }
                }
            }
        }
        println!("roots:{:?}", root);
    }
    println!("{}", dag);
    // println!("{}", prog);
}
