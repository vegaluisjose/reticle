use crate::backend::arch::ultrascale::lut::*;
use vast::v05::ast as verilog;

fn lut_width(ty: Ty) -> u32 {
    match ty {
        Ty::Lut2 => 4,
        Ty::Lut3 => 8,
        Ty::Lut4 => 16,
        Ty::Lut5 => 32,
        Ty::Lut6 => 64,
    }
}

impl From<Lut> for verilog::Stmt {
    fn from(lut: Lut) -> Self {
        let mut inst = verilog::Instance::new(&lut.id(), &lut.ty().to_string());
        let width = lut_width(lut.ty().clone());
        inst.add_param(
            "INIT",
            verilog::Expr::new_ulit_dec(width, &lut.init().to_string()),
        );
        for (i, input) in lut.inputs().iter().enumerate() {
            let port = format!("I{}", i);
            inst.connect(&port, verilog::Expr::new_ref(input));
        }
        inst.connect("O", verilog::Expr::new_ref(&lut.output()));
        verilog::Stmt::from(inst)
    }
}
