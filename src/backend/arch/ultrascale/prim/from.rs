use crate::backend::arch::ultrascale::prim::lut::{Lut, Ty};
use vast::v05::ast::*;

fn width(ty: Ty) -> u32 {
    match ty {
        Ty::Lut2 => 4,
        Ty::Lut3 => 8,
        Ty::Lut4 => 16,
        Ty::Lut5 => 32,
        Ty::Lut6 => 64,
    }
}

impl From<Lut> for Instance {
    fn from(lut: Lut) -> Self {
        let mut inst = Instance::new(&lut.id(), &lut.ty().to_string());
        let width = width(lut.ty().clone());
        inst.add_param("INIT", Expr::new_ulit_dec(width, &lut.init().to_string()));
        for (i, input) in lut.inputs().iter().enumerate() {
            let port = format!("I{}", i);
            inst.connect(&port, Expr::new_ref(input));
        }
        inst.connect("O", Expr::new_ref(&lut.output()));
        inst
    }
}
