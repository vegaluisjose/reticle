use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::arch::ultrascale::reg::RegPrim;

#[derive(Clone, Debug)]
pub enum Prim {
    Reg(RegPrim),
    Lut(LutPrim),
}
