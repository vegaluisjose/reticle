use crate::ml::ast::*;

impl From<OptDsp> for Opt {
    fn from(opt: OptDsp) -> Self {
        Opt::Dsp(opt)
    }
}

impl From<OpLut> for OpMach {
    fn from(op: OpLut) -> Self {
        OpMach::Lut(op)
    }
}

impl From<OpReg> for OpMach {
    fn from(op: OpReg) -> Self {
        OpMach::Reg(op)
    }
}

impl From<OpDsp> for OpMach {
    fn from(op: OpDsp) -> Self {
        OpMach::Dsp(op)
    }
}

impl From<OpCarry> for OpMach {
    fn from(op: OpCarry) -> Self {
        OpMach::Carry(op)
    }
}

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

impl From<InstrMach> for Instr {
    fn from(instr: InstrMach) -> Self {
        Instr::Mach(instr)
    }
}

impl From<InstrWire> for Instr {
    fn from(instr: InstrWire) -> Self {
        Instr::Wire(instr)
    }
}
