use crate::passes::map::partition::tree::*;

impl TreeNode {
    pub fn new_input(id: &str, ty: TreeTy) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op: TreeOp::Input,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Tree {
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
