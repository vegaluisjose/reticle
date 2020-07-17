use crate::passes::select::instr::*;

// using operation and location for estimating
// cost. Maybe we would want to incorporate
// datatype (ty) as well in the future?
pub fn estimate_instr_cost(instr: &DagInstr) -> i32 {
    match (&instr.op, &instr.loc) {
        (DagOp::Add, DagLoc::Unknown) => 9,
        (DagOp::Add, DagLoc::Lut) => 8,
        (DagOp::Add, DagLoc::Dsp) => 2,
        (DagOp::Sub, DagLoc::Unknown) => 9,
        (DagOp::Sub, DagLoc::Lut) => 8,
        (DagOp::Sub, DagLoc::Dsp) => 2,
        (DagOp::Mul, DagLoc::Unknown) => 9,
        (DagOp::Mul, DagLoc::Lut) => 8,
        (DagOp::Mul, DagLoc::Dsp) => 2,
        (DagOp::Reg, DagLoc::Lut) => -4,
        (DagOp::Reg, DagLoc::Dsp) => -1,
        (_, _) => 0,
    }
}
