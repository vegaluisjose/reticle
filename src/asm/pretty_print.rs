use crate::asm::ast::*;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for ExprCoord {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            ExprCoord::Hole => RcDoc::text("??"),
            ExprCoord::Var(n) => RcDoc::as_string(n),
            ExprCoord::Lit(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        self.ty.to_doc().append(
            self.x
                .to_doc()
                .append(RcDoc::text(","))
                .append(RcDoc::space())
                .append(self.y.to_doc())
                .parens(),
        )
    }
}

impl PrettyPrint for InstrPrim {
    fn to_doc(&self) -> RcDoc<()> {
        let id = if self.dst_id().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::as_string(&self.dst_id())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(self.dst_ty().to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
        };
        let attrs = if self.attrs().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.attrs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .brackets()
        };
        let params = if self.params().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.params().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        id.append(RcDoc::as_string(self.op()))
            .append(attrs)
            .append(params)
            .append(RcDoc::space())
            .append(RcDoc::text("@"))
            .append(self.loc().to_doc())
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Std(instr) => instr.to_doc(),
            Instr::Prim(instr) => instr.to_doc(),
        }
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let inputs = if self.inputs().is_empty() {
            RcDoc::nil().parens()
        } else {
            intersperse(
                self.inputs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        let outputs = if self.outputs().is_empty() {
            RcDoc::nil().parens()
        } else {
            intersperse(
                self.outputs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.body()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        let name = RcDoc::text("def")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.id()))
            .append(inputs)
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(outputs);
        block_with_braces(name, body)
    }
}
