use crate::backend::asm::ast::*;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for LocExpr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            LocExpr::Hole => RcDoc::text("??"),
            LocExpr::Var(n) => RcDoc::as_string(n),
            LocExpr::Lit(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("@").append(self.ty.to_doc()).append(
            self.x
                .to_doc()
                .append(RcDoc::text(","))
                .append(RcDoc::space())
                .append(self.y.to_doc())
                .parens(),
        )
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        let id = if self.id().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::as_string(&self.id())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(self.ty().to_doc())
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
        match self {
            Instr::Std {
                op,
                dst: _,
                attrs: _,
                params: _,
            } => id.append(op.to_doc()).append(attrs).append(params),
            Instr::Prim {
                op,
                dst: _,
                attrs: _,
                params: _,
                loc,
            } => id
                .append(RcDoc::as_string(op))
                .append(attrs)
                .append(params)
                .append(RcDoc::space())
                .append(loc.to_doc()),
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
