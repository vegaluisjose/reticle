use crate::ast::*;
use itertools::Itertools;
use pretty::{block_with_braces, intersperse, PrettyPrint, RcDoc};

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Wire(instr) => instr.to_doc(),
            Instr::Prim(instr) => instr.to_doc(),
        }
    }
}

impl PrettyPrint for Sig {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("pat")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.id()))
            .append(self.input().to_doc())
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(self.output().to_doc())
    }
}

impl PrettyPrint for Pat {
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
            self.pat()
                .iter()
                .sorted_by_key(|(id, _)| (*id).clone())
                .map(|(_, pat)| pat.to_doc()),
            RcDoc::hardline(),
        )
    }
}
