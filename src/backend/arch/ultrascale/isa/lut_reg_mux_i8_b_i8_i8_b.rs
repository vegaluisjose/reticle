use crate::backend::arch::ultrascale::assembler::{Assembler, EmitPrim};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::arch::ultrascale::prim::helpers::regs_from_init;
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
        let wire_name = asm.new_variable_name();
        let wire = verilog::Decl::new_wire(&wire_name, 8);
        let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
        for (i, reg) in regs.iter().enumerate() {
            let mut lut = Lut::new_lut3();
            lut.set_id(&asm.new_instance_name());
            lut.set_init("ac");
            lut.add_input_with_index(&tru, i as u32);
            lut.add_input_with_index(&fal, i as u32);
            lut.add_input(&con);
            lut.set_output_with_index(&wire_name, i as u32);
            let mut reg = reg.clone();
            reg.set_id(&asm.new_instance_name());
            reg.set_clock(&asm.clock());
            reg.set_reset(&asm.reset());
            reg.set_en(&en);
            reg.set_input_with_index(&wire_name, i as u32);
            reg.set_output_with_index(&res, i as u32);
            asm.add_instance(verilog::Stmt::from(lut));
            asm.add_instance(verilog::Stmt::from(reg));
        }
        asm.add_wire(verilog::Stmt::from(wire));
    }
}
