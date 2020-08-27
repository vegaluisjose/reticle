use crate::backend::arch::ultrascale::reg::*;
use crate::backend::verilog;

impl From<RegPrim> for verilog::Stmt {
    fn from(reg: RegPrim) -> Self {
        let mut inst = verilog::Instance::new(&reg.id(), &reg.ty().to_string());
        inst.connect("C", verilog::Expr::new_ref(&reg.clock()));
        inst.connect("CE", verilog::Expr::new_ref(&reg.en()));
        inst.connect("D", verilog::Expr::new_ref(&reg.input()));
        inst.connect("Q", verilog::Expr::new_ref(&reg.output()));
        if reg.is_fdre() {
            inst.connect("R", verilog::Expr::new_ref(&reg.reset()));
        } else {
            inst.connect("S", verilog::Expr::new_ref(&reg.reset()));
        }
        verilog::Stmt::from(inst)
    }
}
