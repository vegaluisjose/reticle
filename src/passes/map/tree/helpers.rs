use crate::backend::asm::ast::InstrPrim;
use crate::backend::target::Tile;
use crate::passes::map::tree::*;
use petgraph::visit::Dfs;

impl TreeNode {
    pub fn new(op: TreeOp) -> TreeNode {
        TreeNode {
            id: String::new(),
            ty: TreeTy::Hole,
            op,
            loc: TreeLoc::Hole,
            attrs: Vec::new(),
            matched: false,
            tile: None,
            cost: f32::INFINITY,
        }
    }

    pub fn new_with_cost(id: &str, ty: TreeTy, op: TreeOp, cost: f32) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op,
            loc: TreeLoc::Hole,
            attrs: Vec::new(),
            matched: false,
            tile: None,
            cost,
        }
    }

    pub fn new_input(id: &str, ty: TreeTy) -> TreeNode {
        TreeNode {
            id: id.to_string(),
            ty,
            op: TreeOp::Input,
            loc: TreeLoc::Hole,
            attrs: Vec::new(),
            matched: false,
            tile: None,
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

    pub fn loc(&self) -> &TreeLoc {
        &self.loc
    }

    pub fn instr(&self) -> Option<&InstrPrim> {
        if let Some(tile) = &self.tile {
            Some(tile.instr())
        } else {
            None
        }
    }

    pub fn tile(&self) -> Option<&Tile> {
        self.tile.as_ref()
    }

    pub fn is_matched(&self) -> bool {
        self.matched
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

    pub fn attrs(&self) -> &Vec<TreeExpr> {
        &self.attrs
    }

    pub fn indexed_attr(&self, index: usize) -> &TreeExpr {
        &self.attrs[index]
    }

    pub fn add_attr(&mut self, expr: TreeExpr) {
        self.attrs.push(expr);
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_ty(&mut self, ty: TreeTy) {
        self.ty = ty;
    }

    pub fn set_loc(&mut self, loc: TreeLoc) {
        self.loc = loc;
    }

    pub fn set_cost(&mut self, cost: f32) {
        self.cost = cost;
    }

    pub fn set_matched(&mut self) {
        self.matched = true;
    }

    pub fn set_tile(&mut self, tile: Tile) {
        self.tile = Some(tile);
    }

    pub fn clear_tile(&mut self) {
        self.tile = None;
    }
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root_index: None,
            graph: TreeGraph::new(),
            ctx: TreeCtx::new(),
        }
    }

    pub fn graph(&self) -> &TreeGraph {
        &self.graph
    }

    pub fn root_index(&self) -> Option<TreeIx> {
        self.root_index
    }

    pub fn contains_node_with_id(&self, name: &str) -> bool {
        self.ctx.contains_key(name)
    }

    pub fn add_node(&mut self, name: &str, node: TreeNode) {
        let ix = self.graph.add_node(node);
        // first node is always root
        if self.ctx.is_empty() {
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
