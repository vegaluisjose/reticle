use crate::backend::arch::ultrascale::lut::*;
use crate::backend::verilog;

fn lut_width(ty: Ty) -> u32 {
    match ty {
        Ty::Lut2 => 4,
        Ty::Lut3 => 8,
        Ty::Lut4 => 16,
        Ty::Lut5 => 32,
        Ty::Lut6 => 64,
    }
}

impl From<Expr> for verilog::Expr {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Ref(name) => verilog::Expr::new_ref(&name),
            Expr::Index(name, index) => verilog::Expr::new_bit(&name, index as i32),
        }
    }
}

impl From<Lut> for verilog::Stmt {
    fn from(lut: Lut) -> Self {
        let mut inst = verilog::Instance::new(&lut.id(), &lut.ty().to_string());
        let width = lut_width(lut.ty().clone());
        inst.add_param("INIT", verilog::Expr::new_ulit_hex(width, &lut.init()));
        for (i, input) in lut.inputs().iter().enumerate() {
            let port = format!("I{}", i);
            inst.connect(&port, verilog::Expr::from(input.clone()));
        }
        inst.connect("O", verilog::Expr::from(lut.output().clone()));
        verilog::Stmt::from(inst)
    }
}
