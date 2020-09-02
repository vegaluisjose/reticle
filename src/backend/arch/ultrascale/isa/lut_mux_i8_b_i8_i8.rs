use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutMuxI8BI8I8;

impl Emit for LutMuxI8BI8I8 {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let con = asm.fresh_variable(&params[0]);
        let tru = asm.fresh_variable(&params[1]);
        let fal = asm.fresh_variable(&params[2]);
        let res = asm.fresh_variable(&instr.dst_id());
        for i in 0..8 {
            let mut lut = Lut::new_lut3();
            lut.set_id(&asm.new_instance_name());
            lut.set_init("ac");
            lut.add_input_with_index(&tru, i);
            lut.add_input_with_index(&fal, i);
            lut.add_input(&con);
            lut.set_output_with_index(&res, i);
            asm.add_instance(verilog::Stmt::from(lut));
        }
    }
}
