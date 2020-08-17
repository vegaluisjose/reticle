use crate::passes::map::partition::tree::*;

impl Default for Tree {
    fn default() -> Tree {
        Tree {
            graph: TreeGraph::new(),
            ctx: TreeCtx::new(),
        }
    }
}
