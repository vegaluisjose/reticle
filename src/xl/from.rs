use crate::xl::ast::*;

impl From<BelLut> for Bel {
    fn from(bel: BelLut) -> Self {
        Bel::Lut(bel)
    }
}

impl From<BelReg> for Bel {
    fn from(bel: BelReg) -> Self {
        Bel::Reg(bel)
    }
}

impl From<BelCarry> for Bel {
    fn from(bel: BelCarry) -> Self {
        Bel::Carry(bel)
    }
}

impl From<BelDsp> for Bel {
    fn from(bel: BelDsp) -> Self {
        Bel::Dsp(bel)
    }
}

impl From<InstrBasc> for Instr {
    fn from(instr: InstrBasc) -> Self {
        Instr::Basc(instr)
    }
}

impl From<InstrMach> for Instr {
    fn from(instr: InstrMach) -> Self {
        Instr::Mach(instr)
    }
}
