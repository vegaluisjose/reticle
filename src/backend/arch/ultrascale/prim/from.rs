use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::arch::ultrascale::prim::Prim;
use crate::backend::arch::ultrascale::reg::RegPrim;
use crate::backend::verilog;

impl From<RegPrim> for Prim {
    fn from(reg: RegPrim) -> Self {
        Prim::Reg(reg)
    }
}

impl From<LutPrim> for Prim {
    fn from(lut: LutPrim) -> Self {
        Prim::Lut(lut)
    }
}

impl From<Prim> for verilog::Stmt {
    fn from(prim: Prim) -> Self {
        match prim {
            Prim::Reg(reg) => verilog::Stmt::from(reg),
            Prim::Lut(lut) => verilog::Stmt::from(lut),
        }
    }
}
