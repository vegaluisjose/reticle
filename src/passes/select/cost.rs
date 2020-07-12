use crate::passes::select::instr::*;

// using operation and location for estimating
// cost. Maybe we would want to incorporate
// datatype (ty) as well in the future?
pub fn estimate_instr_cost(instr: &Instr) -> i32 {
    match (&instr.op, &instr.loc) {
        (InstrOp::Add, InstrLoc::Unknown) => 9,
        (InstrOp::Add, InstrLoc::Lut) => 8,
        (InstrOp::Add, InstrLoc::Dsp) => 2,
        (InstrOp::Sub, InstrLoc::Unknown) => 9,
        (InstrOp::Sub, InstrLoc::Lut) => 8,
        (InstrOp::Sub, InstrLoc::Dsp) => 2,
        (InstrOp::Mul, InstrLoc::Unknown) => 9,
        (InstrOp::Mul, InstrLoc::Lut) => 8,
        (InstrOp::Mul, InstrLoc::Dsp) => 2,
        (InstrOp::Reg, InstrLoc::Lut) => -4,
        (InstrOp::Reg, InstrLoc::Dsp) => -1,
        (_, _) => 0,
    }
}