use crate::passes::select::analysis::Analysis;
use std::fmt;

impl fmt::Display for Analysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\ninstr:{}\nprim:{}\nstd:{}\nhole:{}\nlut:{}\ndsp:{}\nlum:{}\nram:{}",
            self.num_instrs(),
            self.num_prims(),
            self.num_stds(),
            self.num_holes(),
            self.num_luts(),
            self.num_dsps(),
            self.num_lums(),
            self.num_rams()
        )
    }
}
