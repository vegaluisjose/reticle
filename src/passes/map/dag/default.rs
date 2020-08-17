use crate::passes::map::dag::*;

impl Default for Dag {
    fn default() -> Dag {
        Dag {
            graph: DagGraph::new(),
            ctx: DagCtx::new(),
            roots: DagCtx::new(),
        }
    }
}
