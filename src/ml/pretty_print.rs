use crate::ml::ast::*;
// use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::util::pretty_print::{PrettyHelper, PrettyPrint};
// use itertools::Itertools;
use pretty::RcDoc;

impl PrettyPrint for OpReg {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpReg::Fdre => RcDoc::text("fdre"),
            OpReg::Fdse => RcDoc::text("fdse"),
        }
    }
}

impl PrettyPrint for OpDsp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpDsp::Add => RcDoc::text("add"),
            OpDsp::MulAdd => RcDoc::text("muladd"),
        }
    }
}

impl PrettyPrint for OpCarry {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpCarry::Carry8 => RcDoc::text("carry8"),
        }
    }
}

impl PrettyPrint for OpLut {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpLut::Lut1 => RcDoc::text("lut1"),
            OpLut::Lut2 => RcDoc::text("lut2"),
            OpLut::Lut3 => RcDoc::text("lut3"),
            OpLut::Lut4 => RcDoc::text("lut4"),
            OpLut::Lut5 => RcDoc::text("lut5"),
            OpLut::Lut6 => RcDoc::text("lut6"),
        }
    }
}

impl PrettyPrint for OptDsp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OptDsp::Ra => RcDoc::text("ra"),
            OptDsp::Rb => RcDoc::text("rb"),
            OptDsp::Rc => RcDoc::text("rc"),
            OptDsp::Rm => RcDoc::text("rm"),
            OptDsp::Rp => RcDoc::text("rp"),
        }
    }
}

impl PrettyPrint for Opt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Opt::Dsp(opt) => opt.to_doc(),
        }
    }
}

impl PrettyPrint for OptVal {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OptVal::UInt(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for BelLut {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelLut::A5 => RcDoc::text("a5lut"),
            BelLut::B5 => RcDoc::text("b5lut"),
            BelLut::C5 => RcDoc::text("c5lut"),
            BelLut::D5 => RcDoc::text("d5lut"),
            BelLut::E5 => RcDoc::text("e5lut"),
            BelLut::F5 => RcDoc::text("f5lut"),
            BelLut::G5 => RcDoc::text("g5lut"),
            BelLut::H5 => RcDoc::text("h5lut"),
            BelLut::A6 => RcDoc::text("a6lut"),
            BelLut::B6 => RcDoc::text("b6lut"),
            BelLut::C6 => RcDoc::text("c6lut"),
            BelLut::D6 => RcDoc::text("d6lut"),
            BelLut::E6 => RcDoc::text("e6lut"),
            BelLut::F6 => RcDoc::text("f6lut"),
            BelLut::G6 => RcDoc::text("g6lut"),
            BelLut::H6 => RcDoc::text("h6lut"),
        }
    }
}

impl PrettyPrint for BelReg {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelReg::A => RcDoc::text("aff"),
            BelReg::B => RcDoc::text("bff"),
            BelReg::C => RcDoc::text("cff"),
            BelReg::D => RcDoc::text("dff"),
            BelReg::E => RcDoc::text("eff"),
            BelReg::F => RcDoc::text("fff"),
            BelReg::G => RcDoc::text("gff"),
            BelReg::H => RcDoc::text("hff"),
            BelReg::A2 => RcDoc::text("aff2"),
            BelReg::B2 => RcDoc::text("bff2"),
            BelReg::C2 => RcDoc::text("cff2"),
            BelReg::D2 => RcDoc::text("dff2"),
            BelReg::E2 => RcDoc::text("eff2"),
            BelReg::F2 => RcDoc::text("fff2"),
            BelReg::G2 => RcDoc::text("gff2"),
            BelReg::H2 => RcDoc::text("hff2"),
        }
    }
}

impl PrettyPrint for BelCarry {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelCarry::Carry8 => RcDoc::text("carry8"),
            BelCarry::Carry4 => RcDoc::text("carry4"),
        }
    }
}

impl PrettyPrint for Bel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Bel::Lut(lut) => lut.to_doc(),
            Bel::Reg(reg) => reg.to_doc(),
            Bel::Carry(carry) => carry.to_doc(),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        let xy = self
            .x()
            .to_doc()
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(self.y().to_doc())
            .parens();
        if let Some(bel) = self.bel() {
            bel.to_doc().append(xy)
        } else {
            xy
        }
    }
}
