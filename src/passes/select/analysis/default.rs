use crate::passes::select::analysis::Analysis;

impl Default for Analysis {
    fn default() -> Analysis {
        Analysis {
            prims: 0,
            stds: 0,
            holes: 0,
            luts: 0,
            dsps: 0,
            lums: 0,
            rams: 0,
        }
    }
}
