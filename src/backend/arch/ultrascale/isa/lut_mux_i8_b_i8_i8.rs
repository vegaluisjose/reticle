use crate::backend::arch::ultrascale::assembler::{Assembler, EmitPrim};
use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutMuxI8BI8I8;

impl EmitPrim for LutMuxI8BI8I8 {
    fn emit_prim(asm: &mut Assembler, instr: asm::InstrPrim) {
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_variable(&params[0]);
        let rhs = asm.fresh_variable(&params[1]);
        let res = asm.fresh_variable(&instr.dst_id());
        for i in 0..8 {
            let mut lut = LutPrim::new_lut3();
            lut.set_id(&asm.new_instance_name());
            lut.set_init("ac");
            lut.add_input_with_index(&lhs, i);
            lut.add_input_with_index(&rhs, i);
            lut.set_output_with_index(&res, i);
            asm.add_lut(verilog::Stmt::from(lut));
        }
    }
}
