use crate::lang::ast::{Instr, Prog};
use crate::passes::map::dag::*;
use petgraph::visit::Dfs;
use petgraph::Direction;

impl DagNodeValue {
    pub fn id(&self) -> String {
        match self {
            DagNodeValue::Inp(port) => port.id(),
            DagNodeValue::Out(port) => port.id(),
            DagNodeValue::Ins(instr) => instr.id(),
        }
    }

    pub fn op_name(&self) -> String {
        match self {
            DagNodeValue::Inp(_) => "in".to_string(),
            DagNodeValue::Out(_) => "out".to_string(),
            DagNodeValue::Ins(instr) => {
                if instr.is_std() {
                    instr.std_op().to_string()
                } else {
                    instr.prim_op().to_string()
                }
            }
        }
    }

    pub fn is_std(&self) -> bool {
        match self {
            DagNodeValue::Ins(instr) => instr.is_std(),
            _ => false,
        }
    }

    pub fn is_prim(&self) -> bool {
        match self {
            DagNodeValue::Ins(instr) => instr.is_prim(),
            _ => false,
        }
    }

    pub fn instr(&self) -> &Instr {
        match self {
            DagNodeValue::Ins(instr) => instr,
            _ => panic!("Error: not an instruction"),
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

    pub fn set_visited(&mut self) {
        self.visited = true;
    }

    pub fn set_root(&mut self) {
        self.root = true;
    }

    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn is_prim(&self) -> bool {
        self.value.is_prim()
    }

    pub fn is_std(&self) -> bool {
        self.value.is_std()
    }

    pub fn instr(&self) -> &Instr {
        self.value.instr()
    }

    pub fn id(&self) -> String {
        self.value.id()
    }
}

impl Dag {
    pub fn add_node(&mut self, name: &str, value: DagNodeValue) {
        let ix = self.graph.add_node(DagNode::new(value));
        self.ctx.insert(name.to_string(), ix);
    }

    pub fn contains_node_with_id(&self, name: &str) -> bool {
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

    pub fn roots(&self) -> &DagCtx {
        &self.roots
    }

    pub fn find_roots(&mut self, prog: &Prog) {
        let mut roots = DagCtx::new();
        // find roots, roots are:
        // 1. Instructions that are primitives (no std)
        // 2. Nodes that have a fanout greater than one (reuse) or are zero (output)
        if let Some(def) = prog.defs().iter().next() {
            for input in def.inputs().iter() {
                if let Some(ix) = self.get_node_index(&input.id()) {
                    let mut visit = Dfs::new(&self.graph, *ix);
                    while let Some(next) = visit.next(&self.graph) {
                        if let Some(node) = self.graph.node_weight(next) {
                            let fanout = self
                                .graph
                                .neighbors_directed(next, Direction::Outgoing)
                                .count();
                            if node.value.is_prim()
                                && fanout != 1
                                && !roots.contains_key(&node.value.id())
                            {
                                roots.insert(node.value.id(), next);
                            }
                        }
                    }
                }
            }
        }
        // mark root nodes
        for (_, root) in roots.iter() {
            if let Some(node) = self.graph.node_weight_mut(*root) {
                node.set_root();
            }
        }
        // store roots
        self.roots = roots;
    }

    pub fn get_incoming_nodes(&self, ix: DagIx) -> DagCtx {
        let mut ctx = DagCtx::new();
        let neighbors = self.graph.neighbors_directed(ix, Direction::Incoming);
        for nix in neighbors {
            if let Some(node) = self.graph.node_weight(nix) {
                if node.is_prim() && !node.is_visited() && !node.is_root() {
                    ctx.insert(node.value.id(), nix);
                }
            }
        }
        ctx
    }
}
