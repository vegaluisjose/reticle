use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::arch::ultrascale::prim::Prim;
use crate::backend::arch::ultrascale::reg::RegPrim;

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
