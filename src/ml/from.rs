use crate::ml::ast::*;

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
