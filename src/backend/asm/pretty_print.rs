use crate::backend::asm::ast::*;
use crate::util::pretty_print::PrettyPrint;
use pretty::RcDoc;

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
        dst_doc.append(RcDoc::as_string(&self.op))
    }
}