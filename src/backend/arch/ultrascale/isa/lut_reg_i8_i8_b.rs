use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::util::regs_from_init;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutRegI8I8B;

impl Emit for LutRegI8I8B {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let val = asm.fresh_variable(&params[0]);
        let en = asm.fresh_variable(&params[1]);
        let res = asm.fresh_variable(&instr.dst_id());
        let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
        for (i, reg) in regs.iter().enumerate() {
            let mut reg = reg.clone();
            reg.set_id(&asm.new_instance_name());
            reg.set_clock(&asm.clock());
            reg.set_reset(&asm.reset());
            reg.set_en(&en);
            reg.set_input_with_index(&val, i as u32);
            reg.set_output_with_index(&res, i as u32);
            asm.add_instance(verilog::Stmt::from(reg));
        }
    }
}
