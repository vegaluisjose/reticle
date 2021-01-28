use crate::tdl::ast::*;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for PatInstr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            PatInstr::Wire(instr) => instr.to_doc(),
            PatInstr::Comp(instr) => instr.to_doc(),
        }
    }
}

impl PrettyPrint for ImpInstr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            ImpInstr::XL(instr) => instr.to_doc(),
        }
    }
}

impl PrettyPrint for PatSig {
    fn to_doc(&self) -> RcDoc<()> {
        let attr_val = vec![
            self.prim().to_doc(),
            RcDoc::as_string(self.area()),
            RcDoc::as_string(self.lat()),
        ];
        let attr = intersperse(
            attr_val.iter().cloned(),
            RcDoc::text(",").append(RcDoc::space()),
        )
        .brackets();
        RcDoc::text("pat")
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

impl PrettyPrint for ImpSig {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("imp")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.id()))
            .append(RcDoc::space())
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
        let pat = intersperse(
            self.pat().iter().map(|(_, pat)| pat.to_doc()),
            RcDoc::hardline(),
        );
        let imp = intersperse(
            self.imp().iter().map(|(_, imp)| imp.to_doc()),
            RcDoc::hardline(),
        );
        if self.imp().is_empty() {
            pat
        } else {
            pat.append(RcDoc::space()).append(imp)
        }
    }
}
