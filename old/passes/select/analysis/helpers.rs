use crate::passes::select::analysis::Analysis;

impl Analysis {
    pub fn num_prims(&self) -> u32 {
        self.prims
    }
    pub fn num_stds(&self) -> u32 {
        self.stds
    }
    pub fn num_instrs(&self) -> u32 {
        self.prims + self.stds
    }
    pub fn num_holes(&self) -> u32 {
        self.holes
    }
    pub fn num_luts(&self) -> u32 {
        self.luts
    }
    pub fn num_dsps(&self) -> u32 {
        self.dsps
    }
    pub fn num_lums(&self) -> u32 {
        self.lums
    }
    pub fn num_rams(&self) -> u32 {
        self.rams
    }
    pub fn inc_prim(&mut self) {
        self.prims += 1;
    }
    pub fn inc_std(&mut self) {
        self.stds += 1;
    }
    pub fn inc_hole(&mut self) {
        self.holes += 1;
    }
    pub fn inc_lut(&mut self) {
        self.luts += 1;
    }
    pub fn inc_dsp(&mut self) {
        self.dsps += 1;
    }
    pub fn inc_lum(&mut self) {
        self.lums += 1;
    }
    pub fn inc_ram(&mut self) {
        self.rams += 1;
    }
}
