use crate::ast::*;
use ir::pretty_print::expr_names;
use prettyprint::{block_with_braces, intersperse, PrettyHelper, PrettyPrint, RcDoc};

impl PrettyPrint for OpCoord {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpCoord::Add => RcDoc::text("+"),
        }
    }
}

impl PrettyPrint for ExprCoord {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            ExprCoord::Any => RcDoc::text("??"),
            ExprCoord::Val(n) => RcDoc::as_string(n),
            ExprCoord::Var(n) => RcDoc::as_string(n),
            ExprCoord::Bin(op, lhs, rhs) => lhs.to_doc().append(op.to_doc()).append(rhs.to_doc()),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        let coord = self
            .x()
            .to_doc()
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(self.y().to_doc());
        self.prim().to_doc().append(coord.parens())
    }
}

impl PrettyPrint for OpAsm {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpAsm::Op(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for InstrAsm {
    fn to_doc(&self) -> RcDoc<()> {
        self.dst()
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(self.op().to_doc())
            .append(expr_names(self.arg()))
            .append(RcDoc::space())
            .append(RcDoc::text("@"))
            .append(self.loc().to_doc())
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Wire(instr) => instr.to_doc(),
            Instr::Asm(instr) => instr.to_doc(),
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
