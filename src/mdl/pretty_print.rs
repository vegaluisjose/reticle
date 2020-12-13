use crate::mdl::ast::*;
// use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::util::pretty_print::PrettyPrint;
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
            BelLut::A5 => RcDoc::text("A5LUT"),
            BelLut::B5 => RcDoc::text("B5LUT"),
            BelLut::C5 => RcDoc::text("C5LUT"),
            BelLut::D5 => RcDoc::text("D5LUT"),
            BelLut::E5 => RcDoc::text("E5LUT"),
            BelLut::F5 => RcDoc::text("F5LUT"),
            BelLut::G5 => RcDoc::text("G5LUT"),
            BelLut::H5 => RcDoc::text("H5LUT"),
            BelLut::A6 => RcDoc::text("A6LUT"),
            BelLut::B6 => RcDoc::text("B6LUT"),
            BelLut::C6 => RcDoc::text("C6LUT"),
            BelLut::D6 => RcDoc::text("D6LUT"),
            BelLut::E6 => RcDoc::text("E6LUT"),
            BelLut::F6 => RcDoc::text("F6LUT"),
            BelLut::G6 => RcDoc::text("G6LUT"),
            BelLut::H6 => RcDoc::text("H6LUT"),
        }
    }
}

impl PrettyPrint for BelReg {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelReg::A => RcDoc::text("AFF"),
            BelReg::B => RcDoc::text("BFF"),
            BelReg::C => RcDoc::text("CFF"),
            BelReg::D => RcDoc::text("DFF"),
            BelReg::E => RcDoc::text("EFF"),
            BelReg::F => RcDoc::text("FFF"),
            BelReg::G => RcDoc::text("GFF"),
            BelReg::H => RcDoc::text("HFF"),
            BelReg::A2 => RcDoc::text("AFF2"),
            BelReg::B2 => RcDoc::text("BFF2"),
            BelReg::C2 => RcDoc::text("CFF2"),
            BelReg::D2 => RcDoc::text("DFF2"),
            BelReg::E2 => RcDoc::text("EFF2"),
            BelReg::F2 => RcDoc::text("FFF2"),
            BelReg::G2 => RcDoc::text("GFF2"),
            BelReg::H2 => RcDoc::text("HFF2"),
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