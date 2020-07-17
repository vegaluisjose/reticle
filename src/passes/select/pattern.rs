use crate::passes::select::instr::*;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub name: String,
    pub instr: Vec<DagInstr>,
    pub cost: i32,
}

// hardcoded patterns for now, this could be generated
// from a target-description (td) file

fn dsp_i8_mul_r_add() -> Pattern {
    let mut instr: Vec<DagInstr> = Vec::new();
    instr.push(DagInstr::new(DagOp::Add, DagTy::SInt(8), DagLoc::Dsp));
    instr.push(DagInstr::new(DagOp::Reg, DagTy::SInt(8), DagLoc::Dsp));
    instr.push(DagInstr::new(DagOp::Mul, DagTy::SInt(8), DagLoc::Dsp));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    instr.push(DagInstr::new(DagOp::Any, DagTy::Bool, DagLoc::Lut));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    Pattern {
        name: "dsp_i8_mul_r_add".to_string(),
        instr: instr,
        cost: 1,
    }
}

fn dsp_i8_mul() -> Pattern {
    let mut instr: Vec<DagInstr> = Vec::new();
    instr.push(DagInstr::new(DagOp::Mul, DagTy::SInt(8), DagLoc::Dsp));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    Pattern {
        name: "dsp_i8_mul".to_string(),
        instr: instr,
        cost: 4,
    }
}

fn dsp_i8_add() -> Pattern {
    let mut instr: Vec<DagInstr> = Vec::new();
    instr.push(DagInstr::new(DagOp::Add, DagTy::SInt(8), DagLoc::Dsp));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    instr.push(DagInstr::new(DagOp::Any, DagTy::SInt(8), DagLoc::Lut));
    Pattern {
        name: "dsp_i8_add".to_string(),
        instr: instr,
        cost: 4,
    }
}

pub fn patterns() -> Vec<Pattern> {
    vec![dsp_i8_add(), dsp_i8_mul(), dsp_i8_mul_r_add()]
}
