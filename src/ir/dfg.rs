use crate::ir::ast::*;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;
use std::fmt;

pub type DfgGraph = Graph<DfgNode, DfgEdge>;
pub type DfgCtx = HashMap<String, NodeIndex>;

#[derive(Clone, Debug)]
pub struct DfgNode {
    pub instr: Instr,
}

#[derive(Clone, Debug, Default)]
pub struct DfgEdge;

#[derive(Clone, Debug, Default)]
pub struct Dfg {
    pub graph: DfgGraph,
    pub ctx: DfgCtx,
}

impl DfgNode {
    pub fn new(instr: Instr) -> Self {
        DfgNode { instr }
    }
    pub fn instr(&self) -> &Instr {
        &self.instr
    }
}

impl Dfg {
    pub fn add_node<S>(&mut self, name: S, instr: Instr)
    where
        S: AsRef<str>,
    {
        let ix = self.graph.add_node(DfgNode::new(instr));
        self.ctx.insert(name.as_ref().to_string(), ix);
    }
}

fn inp_from_term(term: ExprTerm) -> Instr {
    Instr::from(InstrWire {
        op: OpWire::Inp,
        dst: Expr::from(term),
        attr: Expr::default(),
        arg: Expr::default(),
    })
}

impl From<Def> for Dfg {
    fn from(def: Def) -> Self {
        let mut dfg = Dfg::default();
        let term: Vec<ExprTerm> = def.sig().input().clone().into();
        for e in term {
            let instr = inp_from_term(e.clone());
            if let Some(id) = e.id() {
                dfg.add_node(id, instr);
            }
        }
        dfg
    }
}

impl From<Prog> for Dfg {
    fn from(prog: Prog) -> Self {
        if let Some(def) = prog.get("main") {
            Dfg::from(def.clone())
        } else {
            Dfg::default()
        }
    }
}

impl fmt::Display for DfgNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.instr() {
            Instr::Wire(instr) => write!(f, "{} [{}]", self.instr().dst(), instr.op()),
            Instr::Comp(instr) => write!(f, "{} [{}]", self.instr().dst(), instr.op()),
            Instr::Call(instr) => write!(f, "{} [{}]", self.instr().dst(), instr.op()),
        }
    }
}

impl fmt::Display for DfgEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Dfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
}
