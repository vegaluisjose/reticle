use crate::ast::*;
use itertools::Itertools;
use pretty::{block_with_braces, intersperse, PrettyHelper, PrettyPrint, RcDoc};

impl PrettyPrint for Sig {
    fn to_doc(&self) -> RcDoc<()> {
        let attr_val = vec![RcDoc::as_string(self.area()), RcDoc::as_string(self.perf())];
        let attr = intersperse(
            attr_val.iter().cloned(),
            RcDoc::text(",").append(RcDoc::space()),
        )
        .brackets();
        RcDoc::text("imp")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.id()))
            .append(attr)
            .append(self.input().to_doc())
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(self.output().to_doc())
    }
}

impl PrettyPrint for Imp {
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

impl PrettyPrint for Target {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.imp()
                .iter()
                .sorted_by_key(|(id, _)| (*id).clone())
                .map(|(_, imp)| imp.to_doc()),
            RcDoc::hardline(),
        )
    }
}
