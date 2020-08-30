use crate::backend::arch::ultrascale::prim::ast::*;
use crate::backend::verilog;

fn lut_width(ty: LutTy) -> u32 {
    match ty {
        LutTy::Lut2 => 4,
        LutTy::Lut3 => 8,
        LutTy::Lut4 => 16,
        LutTy::Lut5 => 32,
        LutTy::Lut6 => 64,
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

impl From<Reg> for verilog::Stmt {
    fn from(reg: Reg) -> Self {
        let mut inst = verilog::Instance::new(&reg.id(), &reg.ty().to_string());
        inst.connect("C", verilog::Expr::from(reg.clock().clone()));
        inst.connect("CE", verilog::Expr::from(reg.en().clone()));
        inst.connect("D", verilog::Expr::from(reg.input().clone()));
        inst.connect("Q", verilog::Expr::from(reg.output().clone()));
        if reg.is_fdre() {
            inst.connect("R", verilog::Expr::from(reg.reset().clone()));
        } else {
            inst.connect("S", verilog::Expr::from(reg.reset().clone()));
        }
        verilog::Stmt::from(inst)
    }
}
