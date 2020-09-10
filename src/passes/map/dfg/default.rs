use crate::passes::map::dfg::*;

impl Default for Dfg {
    fn default() -> Dfg {
        Dfg {
            graph: DfgGraph::new(),
            ctx: DfgCtx::new(),
            roots: DfgCtx::new(),
        }
    }
}
