use crate::backend::arch::ultrascale::assembler::{Assembler, EmitPrim};
use crate::backend::arch::ultrascale::prim::ast::{Lut, Reg};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutRegMuxI8BI8I8B;

impl EmitPrim for LutRegMuxI8BI8I8B {
    fn emit_prim(asm: &mut Assembler, instr: asm::InstrPrim) {
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let con = asm.fresh_variable(&params[0]);
        let tru = asm.fresh_variable(&params[1]);
        let fal = asm.fresh_variable(&params[2]);
        let en = asm.fresh_variable(&params[3]);
        let res = asm.fresh_variable(&instr.dst_id());
        for i in 0..8 {
            let wire_name = asm.new_variable_name();
            let wire = verilog::Decl::new_wire(&wire_name, 1);
            let mut lut = Lut::new_lut3();
            lut.set_id(&asm.new_instance_name());
            lut.set_init("ac");
            lut.add_input_with_index(&tru, i);
            lut.add_input_with_index(&fal, i);
            lut.add_input(&con);
            lut.set_output(&wire_name);
            let mut reg = Reg::new_fdre();
            reg.set_id(&asm.new_instance_name());
            reg.set_clock(&asm.clock());
            reg.set_reset(&asm.reset());
            reg.set_en(&en);
            reg.set_input(&wire_name);
            reg.set_output_with_index(&res, i);
            asm.add_lut(verilog::Stmt::from(lut));
            asm.add_reg(verilog::Stmt::from(reg));
            asm.add_wire(verilog::Stmt::from(wire));
        }
    }
}
