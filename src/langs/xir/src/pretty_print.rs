use crate::ast::*;
use ir::pretty_print::{expr_attrs, expr_names};
use prettyprint::{block_with_braces, intersperse, PrettyHelper, PrettyPrint, RcDoc};

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
            OpMach::CarryAdd => RcDoc::text("carryadd"),
            OpMach::CarrySub => RcDoc::text("carrysub"),
            OpMach::VecAddRegA => RcDoc::text("vaddrega"),
            OpMach::VecAdd => RcDoc::text("vadd"),
            OpMach::VecSub => RcDoc::text("vsub"),
            OpMach::VecMul => RcDoc::text("vmul"),
            OpMach::Mul => RcDoc::text("mul"),
            OpMach::MulAdd => RcDoc::text("muladd"),
            OpMach::MulAddRegA => RcDoc::text("muladdrega"),
            OpMach::MulAddRegACi => RcDoc::text("muladdregaci"),
            OpMach::MulAddRegACo => RcDoc::text("muladdregaco"),
            OpMach::MulAddRegACio => RcDoc::text("muladdregacio"),
            OpMach::Lram => RcDoc::text("lram"),
            OpMach::Bram => RcDoc::text("bram"),
            OpMach::Lrom => RcDoc::text("lrom"),
            OpMach::Brom => RcDoc::text("brom"),
        }
    }
}

impl PrettyPrint for BelLut {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelLut::A5 => RcDoc::text("a5"),
            BelLut::B5 => RcDoc::text("b5"),
            BelLut::C5 => RcDoc::text("c5"),
            BelLut::D5 => RcDoc::text("d5"),
            BelLut::E5 => RcDoc::text("e5"),
            BelLut::F5 => RcDoc::text("f5"),
            BelLut::G5 => RcDoc::text("g5"),
            BelLut::H5 => RcDoc::text("h5"),
            BelLut::A6 => RcDoc::text("a6"),
            BelLut::B6 => RcDoc::text("b6"),
            BelLut::C6 => RcDoc::text("c6"),
            BelLut::D6 => RcDoc::text("d6"),
            BelLut::E6 => RcDoc::text("e6"),
            BelLut::F6 => RcDoc::text("f6"),
            BelLut::G6 => RcDoc::text("g6"),
            BelLut::H6 => RcDoc::text("h6"),
        }
    }
}

impl PrettyPrint for BelReg {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelReg::A => RcDoc::text("a"),
            BelReg::B => RcDoc::text("b"),
            BelReg::C => RcDoc::text("c"),
            BelReg::D => RcDoc::text("d"),
            BelReg::E => RcDoc::text("e"),
            BelReg::F => RcDoc::text("f"),
            BelReg::G => RcDoc::text("g"),
            BelReg::H => RcDoc::text("h"),
            BelReg::A2 => RcDoc::text("a2"),
            BelReg::B2 => RcDoc::text("b2"),
            BelReg::C2 => RcDoc::text("c2"),
            BelReg::D2 => RcDoc::text("d2"),
            BelReg::E2 => RcDoc::text("e2"),
            BelReg::F2 => RcDoc::text("f2"),
            BelReg::G2 => RcDoc::text("g2"),
            BelReg::H2 => RcDoc::text("h2"),
        }
    }
}

impl PrettyPrint for BelCarry {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelCarry::Carry8 => RcDoc::text("c8"),
            BelCarry::Carry4 => RcDoc::text("c4"),
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

impl PrettyPrint for BelBlock {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelBlock::U => RcDoc::text("l"),
            BelBlock::L => RcDoc::text("u"),
        }
    }
}

impl PrettyPrint for BelLum {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            BelLum::H6 => RcDoc::text("h6"),
        }
    }
}

impl PrettyPrint for Bel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Bel::Lut(b) => b.to_doc(),
            Bel::Reg(b) => b.to_doc(),
            Bel::Carry(b) => b.to_doc(),
            Bel::Dsp(b) => b.to_doc(),
            Bel::Block(b) => b.to_doc(),
            Bel::Lum(b) => b.to_doc(),
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
            .append(self.y().to_doc());
        self.bel().to_doc().append(xy.parens())
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
            .append(expr_attrs(self.attr()))
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
