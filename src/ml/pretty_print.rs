use crate::ir::pretty_print::{expr_attrs, expr_names};
use crate::ml::ast::*;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
// use itertools::Itertools;
use pretty::RcDoc;

impl PrettyPrint for OpBasc {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpBasc::Id => RcDoc::text("id"),
            OpBasc::Gnd => RcDoc::text("gnd"),
            OpBasc::Vcc => RcDoc::text("vcc"),
            OpBasc::Ext => RcDoc::text("ext"),
            OpBasc::Cat => RcDoc::text("cat"),
        }
    }
}

impl PrettyPrint for OpMach {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpMach::Lut1 => RcDoc::text("lut1"),
            OpMach::Lut2 => RcDoc::text("lut2"),
            OpMach::Lut3 => RcDoc::text("lut3"),
            OpMach::Lut4 => RcDoc::text("lut4"),
            OpMach::Lut5 => RcDoc::text("lut5"),
            OpMach::Lut6 => RcDoc::text("lut6"),
            OpMach::Fdre => RcDoc::text("fdre"),
            OpMach::Fdse => RcDoc::text("fdse"),
            OpMach::Dsp => RcDoc::text("dsp"),
            OpMach::Carry => RcDoc::text("carry"),
        }
    }
}

impl PrettyPrint for Opt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Opt::RegA => RcDoc::text("ra"),
            Opt::RegB => RcDoc::text("rb"),
            Opt::RegC => RcDoc::text("rc"),
            Opt::RegM => RcDoc::text("rm"),
            Opt::RegP => RcDoc::text("rp"),
            Opt::Op => RcDoc::text("op"),
            Opt::Table => RcDoc::text("tbl"),
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

impl PrettyPrint for OptVal {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OptVal::UInt(n) => RcDoc::as_string(n),
            OptVal::Op(op) => op.to_doc(),
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

impl PrettyPrint for BelDsp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelDsp::Alu => RcDoc::text("alu"),
        }
    }
}

impl PrettyPrint for Bel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Bel::Lut(lut) => lut.to_doc(),
            Bel::Reg(reg) => reg.to_doc(),
            Bel::Carry(carry) => carry.to_doc(),
            Bel::Dsp(dsp) => dsp.to_doc(),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        let xy = self.x().to_doc()
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(self.y().to_doc());
        self.bel().to_doc().append(xy.parens())
    }
}

impl PrettyPrint for OptMap {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.iter()
                .map(|(opt, val)| opt.to_doc().append(RcDoc::text("=")).append(val.to_doc())),
            RcDoc::text(",").append(RcDoc::space()),
        )
    }
}

impl PrettyPrint for InstrBasc {
    fn to_doc(&self) -> RcDoc<()> {
        let arg = match self.op() {
            OpBasc::Gnd | OpBasc::Vcc => RcDoc::nil().parens(),
            _ => expr_names(self.arg()),
        };
        self.dst()
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(self.op().to_doc())
            .append(expr_attrs(self.attr()))
            .append(arg)
    }
}

impl PrettyPrint for InstrMach {
    fn to_doc(&self) -> RcDoc<()> {
        let opt = if self.opt().is_empty() {
            RcDoc::nil()
        } else {
            self.opt().to_doc().brackets()
        };
        let loc = if let Some(loc) = self.loc() {
            RcDoc::space().append(RcDoc::text("@")).append(loc.to_doc())
        } else {
            RcDoc::nil()
        };
        self.dst()
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(self.op().to_doc())
            .append(opt)
            .append(expr_names(self.arg()))
            .append(loc)
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Basc(basc) => basc.to_doc(),
            Instr::Mach(mach) => mach.to_doc(),
        }
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let sig = self.sig().to_doc();
        let body = intersperse(
            self.body()
                .iter()
                .map(|i| i.to_doc().append(RcDoc::text(";"))),
            RcDoc::hardline(),
        );
        block_with_braces(sig, body)
    }
}
