use crate::passes::select::instr::*;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub name: String,
    pub instrs: Vec<Instr>,
    pub cost: i32,
}

// hardcoded patterns for now, this could be generated
// from a target-description (td) file

fn dsp_i8_mul_r_add () -> Pattern {
    let mut instrs: Vec<Instr> = Vec::new();
    instrs.push(Instr::new(InstrOp::Add, InstrTy::SInt(8), InstrLoc::Dsp));
    instrs.push(Instr::new(InstrOp::Reg, InstrTy::SInt(8), InstrLoc::Dsp));
    instrs.push(Instr::new(InstrOp::Mul, InstrTy::SInt(8), InstrLoc::Dsp));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::Bool, InstrLoc::Any));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    Pattern {
        name: "dsp_i8_mul_r_add".to_string(),
        instrs: instrs,
        cost: 1,
    }
}

fn dsp_i8_mul () -> Pattern {
    let mut instrs: Vec<Instr> = Vec::new();
    instrs.push(Instr::new(InstrOp::Mul, InstrTy::SInt(8), InstrLoc::Dsp));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    Pattern {
        name: "dsp_i8_mul".to_string(),
        instrs: instrs,
        cost: 4,
    }
}

fn dsp_i8_add () -> Pattern {
    let mut instrs: Vec<Instr> = Vec::new();
    instrs.push(Instr::new(InstrOp::Add, InstrTy::SInt(8), InstrLoc::Dsp));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    instrs.push(Instr::new(InstrOp::Any, InstrTy::SInt(8), InstrLoc::Any));
    Pattern {
        name: "dsp_i8_add".to_string(),
        instrs: instrs,
        cost: 4,
    }
}

pub fn patterns() -> Vec<Pattern> {
    vec![
        dsp_i8_add(),
        dsp_i8_mul(),
        dsp_i8_mul_r_add(),
    ]
}