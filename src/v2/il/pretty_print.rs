use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::v2::il::ast::*;
use itertools::Itertools;
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Var => RcDoc::text("??"),
            Ty::Bool => RcDoc::text("bool"),
            Ty::UInt(width) => RcDoc::text("u").append(RcDoc::as_string(width)),
            Ty::SInt(width) => RcDoc::text("i").append(RcDoc::as_string(width)),
            Ty::Vector(dtype, len) => dtype.to_doc().append(RcDoc::as_string(len)).comps(),
        }
    }
}

impl PrettyPrint for ExprTup {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.exprs().iter().map(|e| e.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        )
        .parens()
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Val(v) => RcDoc::as_string(v),
            Expr::Name(n, ty) => RcDoc::as_string(n)
                .append(RcDoc::text(":"))
                .append(ty.to_doc()),
            Expr::Tup(tup) => tup.to_doc(),
        }
    }
}

impl PrettyPrint for Sig {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text(self.id())
            .append(self.inputs().to_doc())
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(self.outputs().to_doc())
    }
}

impl PrettyPrint for Def {
    fn to_doc(&self) -> RcDoc<()> {
        let sig = self.sig().to_doc();
        block_with_braces(sig, RcDoc::nil())
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let main = self.get("main");
        if self.defs().len() == 1 {
            main.to_doc()
        } else {
            intersperse(
                self.defs()
                    .iter()
                    .filter(|(id, _)| id.as_str() != "main")
                    .sorted_by_key(|(id, _)| (*id).clone())
                    .map(|(_, def)| def.to_doc()),
                RcDoc::hardline(),
            )
            .append(RcDoc::hardline())
            .append(main.to_doc())
        }
    }
}
