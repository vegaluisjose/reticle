use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutAndBBB;

impl Emit for LutAndBBB {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let inst = asm.new_instance_name();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let in_0 = asm.replace_variable(&params[0]);
        let in_1 = asm.replace_variable(&params[1]);
        let out = asm.replace_variable(&instr.dst_id());
        let mut lut = LutPrim::new_lut2();
        lut.set_init("8");
        lut.set_id(&inst);
        lut.add_input(&in_0);
        lut.add_input(&in_1);
        lut.set_output(&out);
        asm.add_lut(verilog::Stmt::from(lut));
    }
}
