use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::util::regs_from_init;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutReg;

impl Emit for LutReg {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let val = asm.fresh_scalar_variable(&params[0]);
        let en = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
        let width = instr.dst_ty().width();
        for (i, reg) in regs.iter().enumerate() {
            let mut reg = reg.clone();
            reg.set_id(&asm.new_instance_name());
            reg.set_input("clock", &asm.clock());
            reg.set_input("reset", &asm.reset());
            reg.set_input("en", &en);
            if width == 1 {
                reg.set_input("a", &val);
                reg.set_output("y", &res);
            } else {
                reg.set_input_with_index("a", &val, i as u32);
                reg.set_output_with_index("y", &res, i as u32);
            }
            asm.add_instance(verilog::Stmt::from(reg));
        }
    }
}
