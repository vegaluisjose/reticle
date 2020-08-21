use crate::backend::asm::ast::Instr;
use crate::passes::map::partition::tree::*;
use petgraph::visit::Dfs;

impl TreeNode {
    pub fn new(id: &str, ty: TreeTy, op: TreeOp) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op,
            matched: false,
            instr: None,
            cost: f32::INFINITY,
        }
    }

    pub fn new_with_cost(id: &str, ty: TreeTy, op: TreeOp, cost: u32) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op,
            matched: false,
            instr: None,
            cost: cost as f32,
        }
    }

    pub fn new_input(id: &str, ty: TreeTy) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op: TreeOp::Input,
            matched: false,
            instr: None,
            cost: 0 as f32,
        }
    }

    pub fn is_input(&self) -> bool {
        self.op == TreeOp::Input
    }

    pub fn ty(&self) -> &TreeTy {
        &self.ty
    }

    pub fn op(&self) -> &TreeOp {
        &self.op
    }

    pub fn is_matched(&self) -> bool {
        self.matched
    }

    pub fn set_matched(&mut self) {
        self.matched = true;
    }

    pub fn set_instr(&mut self, instr: Instr) {
        self.instr = Some(instr);
    }

    pub fn clear_instr(&mut self) {
        self.instr = None;
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn cost(&self) -> f32 {
        self.cost
    }

    pub fn has_infinity_cost(&self) -> bool {
        self.cost == f32::INFINITY
    }
}

impl Tree {
    pub fn new(root: &str) -> Tree {
        Tree {
            root_id: root.to_string(),
            root_index: None,
            graph: TreeGraph::new(),
            ctx: TreeCtx::new(),
        }
    }

    pub fn graph(&self) -> &TreeGraph {
        &self.graph
    }

    pub fn root_id(&self) -> String {
        self.root_id.to_string()
    }

    pub fn root_index(&self) -> Option<TreeIx> {
        self.root_index
    }

    pub fn contains_node_with_id(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
    }

    pub fn add_node(&mut self, name: &str, node: TreeNode) {
        let ix = self.graph.add_node(node);
        if name == self.root_id.as_str() && self.ctx.is_empty() {
            self.root_index = Some(ix);
        }
        self.ctx.insert(name.to_string(), ix);
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(from_ix) = self.ctx.get(from) {
            if let Some(to_ix) = self.ctx.get(to) {
                if self.graph.find_edge(*from_ix, *to_ix).is_none() {
                    self.graph.add_edge(*from_ix, *to_ix, TreeEdge::default());
                }
            }
        }
    }

    pub fn estimate_cost_from_index(&self, start: TreeIx) -> f32 {
        let mut total: f32 = 0.0;
        let mut visit = Dfs::new(&self.graph, start);
        while let Some(ix) = visit.next(&self.graph) {
            if let Some(node) = self.graph.node_weight(ix) {
                if node.has_infinity_cost() {
                    total = f32::INFINITY;
                    break;
                } else {
                    total += node.cost();
                }
            }
        }
        total
    }

    pub fn estimate_cost(&self) -> f32 {
        self.estimate_cost_from_index(self.root_index().unwrap())
    }
}
