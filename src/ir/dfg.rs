use crate::ir::ast::*;
use crate::util::errors::Error;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;
use std::convert::TryFrom;
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
    pub fn graph(&self) -> &DfgGraph {
        &self.graph
    }
    pub fn get_node_index<S>(&self, name: S) -> Option<&NodeIndex>
    where
        S: AsRef<str>,
    {
        self.ctx.get(name.as_ref())
    }
    pub fn add_node<S>(&mut self, name: S, instr: Instr)
    where
        S: AsRef<str>,
    {
        let ix = self.graph.add_node(DfgNode::new(instr));
        self.ctx.insert(name.as_ref().to_string(), ix);
    }
    pub fn add_edge<S>(&mut self, from: S, to: S)
    where
        S: AsRef<str>,
    {
        if let Some(from_ix) = self.ctx.get(from.as_ref()) {
            if let Some(to_ix) = self.ctx.get(to.as_ref()) {
                if self.graph.find_edge(*from_ix, *to_ix).is_none() {
                    self.graph.add_edge(*from_ix, *to_ix, DfgEdge::default());
                }
            }
        }
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

impl TryFrom<Def> for Dfg {
    type Error = Error;
    fn try_from(def: Def) -> Result<Self, Self::Error> {
        let mut dfg = Dfg::default();
        let term: Vec<ExprTerm> = def.sig().input().clone().into();
        // create input nodes
        for expr in term {
            let instr = inp_from_term(expr.clone());
            if let Some(id) = expr.id() {
                dfg.add_node(id, instr);
            }
        }
        // create instr dst nodes
        for instr in def.body().iter() {
            if let Some(expr) = instr.dst().term() {
                if let Some(out) = expr.id() {
                    dfg.add_node(out.clone(), instr.clone());
                }
            }
        }
        // create edges
        for instr in def.body().iter() {
            if let Some(expr) = instr.dst().term() {
                if let Some(out) = expr.id() {
                    let arg: Vec<ExprTerm> = instr.arg().clone().into();
                    for a in arg {
                        if let Some(inp) = a.id() {
                            dfg.add_edge(inp, out.clone());
                        }
                    }
                }
            }
        }
        Ok(dfg)
    }
}

impl TryFrom<Prog> for Dfg {
    type Error = Error;
    fn try_from(prog: Prog) -> Result<Self, Self::Error> {
        if let Some(def) = prog.get("main") {
            Ok(Dfg::try_from(def.clone())?)
        } else {
            Err(Error::new_conv_error("dfg, prog must have main"))
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
