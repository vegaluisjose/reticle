use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::arch::ultrascale::prim::Prim;
use crate::backend::asm::ast as asm;

pub fn lut_and_bool_bool_bool(instr: asm::Instr) -> Vec<Prim> {
    let mut prims: Vec<Prim> = Vec::new();
    let mut lut = LutPrim::new_lut2();
    lut.add_input(&instr.params()[0].id());
    lut.add_input(&instr.params()[1].id());
    lut.set_output(&instr.id());
    lut.set_init(8);
    prims.push(Prim::from(lut));
    prims
}
