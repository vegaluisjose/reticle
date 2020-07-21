use crate::backend::asm::ast::*;
use crate::util::pretty_print::PrettyPrint;
use pretty::RcDoc;

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(n, _) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        let dst_doc = match &self.dst {
            Some(n) => RcDoc::as_string(n)
                .append(RcDoc::space())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(self.ty.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space()),
            None => RcDoc::nil(),
        };
        let params_doc = match self.params.is_empty() {
            true => RcDoc::nil(),
            false => RcDoc::text("(")
                .append(RcDoc::intersperse(
                    self.params.iter().map(|p| p.to_doc()),
                    RcDoc::text(",").append(RcDoc::space()),
                ))
                .append(RcDoc::text(")")),
        };
        let op_doc = RcDoc::as_string(&self.op);
        dst_doc
            .append(op_doc)
            .append(params_doc)
    }
}