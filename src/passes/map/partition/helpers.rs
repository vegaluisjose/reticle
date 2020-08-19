use crate::passes::map::partition::tree::*;
use crate::backend::asm::ast::Instr;

impl TreeNode {
    pub fn new(id: &str, ty: TreeTy, op: TreeOp) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op,
            matched: false,
            instr: None,
            cost: std::f32::INFINITY,
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
}

impl Tree {
    pub fn new(root: &str) -> Tree {
        Tree {
            root_id: root.to_string(),
            graph: TreeGraph::new(),
            ctx: TreeCtx::new(),
        }
    }

    pub fn root_id(&self) -> String {
        self.root_id.to_string()
    }

    pub fn contains_node(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
    }

    pub fn add_node(&mut self, name: &str, node: TreeNode) {
        let ix = self.graph.add_node(node);
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
}
