use crate::backend::arch::ultrascale::prim::lut::Lut;
use vast::v05::ast::*;

impl From<Lut> for Instance {
    fn from(lut: Lut) -> Self {
        Instance::new(&lut.id(), &lut.ty().to_string())
    }
}
