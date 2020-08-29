use crate::backend::arch::ultrascale::assembler::{Assembler, EmitPrim};
use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

// wire [7:0]a;
// wire [7:0]b;
// wire y;
// wire y_INST_0_i_1_n_0;
// wire y_INST_0_i_2_n_0;

// LUT6 #(
//   .INIT(64'h9009000000000000))
//   y_INST_0
//      (.I0(b[7]),
//       .I1(a[7]),
//       .I2(b[6]),
//       .I3(a[6]),
//       .I4(y_INST_0_i_1_n_0),
//       .I5(y_INST_0_i_2_n_0),
//       .O(y));
// LUT6 #(
//   .INIT(64'h9009000000009009))
//   y_INST_0_i_1
//      (.I0(a[3]),
//       .I1(b[3]),
//       .I2(b[5]),
//       .I3(a[5]),
//       .I4(b[4]),
//       .I5(a[4]),
//       .O(y_INST_0_i_1_n_0));
// LUT6 #(
//   .INIT(64'h9009000000009009))
//   y_INST_0_i_2
//      (.I0(a[0]),
//       .I1(b[0]),
//       .I2(b[2]),
//       .I3(a[2]),
//       .I4(b[1]),
//       .I5(a[1]),
//       .O(y_INST_0_i_2_n_0));

#[derive(Clone, Debug)]
pub struct LutEqI8I8;

impl EmitPrim for LutEqI8I8 {
    fn emit_prim(asm: &mut Assembler, _: asm::InstrPrim) {
        // let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        // let lhs = asm.fresh_variable(&params[0]);
        // let rhs = asm.fresh_variable(&params[1]);
        // let res = asm.fresh_variable(&instr.dst_id());
        let mut lut_0 = LutPrim::new_lut6();
        let mut lut_1 = LutPrim::new_lut6();
        let mut lut_2 = LutPrim::new_lut6();
        lut_0.set_id(&asm.new_instance_name());
        lut_1.set_id(&asm.new_instance_name());
        lut_2.set_id(&asm.new_instance_name());
        lut_0.set_init("9009000000000000");
        lut_1.set_init("9009000000009009");
        lut_2.set_init("9009000000009009");
        asm.add_lut(verilog::Stmt::from(lut_0));
        asm.add_lut(verilog::Stmt::from(lut_1));
        asm.add_lut(verilog::Stmt::from(lut_2));
    }
}
