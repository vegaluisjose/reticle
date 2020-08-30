use crate::backend::arch::ultrascale::reg::*;
use crate::backend::verilog;

impl From<Expr> for verilog::Expr {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Ref(name) => verilog::Expr::new_ref(&name),
            Expr::Index(name, index) => verilog::Expr::new_bit(&name, index as i32),
        }
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
