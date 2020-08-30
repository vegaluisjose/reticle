use crate::backend::arch::ultrascale::assembler::{Assembler, EmitPrim};
use crate::backend::arch::ultrascale::lut::Lut;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutAndBBB;

impl EmitPrim for LutAndBBB {
    fn emit_prim(asm: &mut Assembler, instr: asm::InstrPrim) {
        let id = asm.new_instance_name();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let in_0 = asm.fresh_variable(&params[0]);
        let in_1 = asm.fresh_variable(&params[1]);
        let out = asm.fresh_variable(&instr.dst_id());
        let mut lut = Lut::new_lut2();
        lut.set_init("8");
        lut.set_id(&id);
        lut.add_input(&in_0);
        lut.add_input(&in_1);
        lut.set_output(&out);
        asm.add_lut(verilog::Stmt::from(lut));
    }
}
