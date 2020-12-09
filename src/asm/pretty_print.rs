use crate::asm::ast::*;
use crate::ir::pretty_print::expr_names;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for ExprCoord {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            ExprCoord::Any => RcDoc::text("??"),
            ExprCoord::Val(n) => RcDoc::as_string(n),
            ExprCoord::Var(n) => RcDoc::as_string(n),
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

impl PrettyPrint for AsmOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            AsmOp::Op(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for InstrAsm {
    fn to_doc(&self) -> RcDoc<()> {
        let attr = if self.attr().is_tup() && self.attr().tup().is_empty() {
            RcDoc::nil()
        } else {
            self.attr().to_doc().brackets()
        };
        self.dst()
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(self.op().to_doc())
            .append(attr)
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
