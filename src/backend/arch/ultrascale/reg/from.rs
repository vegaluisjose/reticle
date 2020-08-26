use crate::backend::arch::ultrascale::reg::*;
use vast::v05::ast as Verilog;

impl From<Reg> for Verilog::Parallel {
    fn from(reg: Reg) -> Self {
        let mut inst = Verilog::Instance::new(&reg.id(), &reg.ty().to_string());
        inst.connect("C", Verilog::Expr::new_ref(&reg.clock()));
        inst.connect("CE", Verilog::Expr::new_ref(&reg.en()));
        inst.connect("D", Verilog::Expr::new_ref(&reg.input()));
        inst.connect("Q", Verilog::Expr::new_ref(&reg.output()));
        if reg.is_fdre() {
            inst.connect("R", Verilog::Expr::new_ref(&reg.reset()));
        } else {
            inst.connect("S", Verilog::Expr::new_ref(&reg.reset()));
        }
        Verilog::Parallel::from(inst)
    }
}
