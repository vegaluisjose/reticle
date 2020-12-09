use crate::tdl::ast::*;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Wire(instr) => instr.to_doc(),
            Instr::Comp(instr) => instr.to_doc(),
        }
    }
}

impl PrettyPrint for Sig {
    fn to_doc(&self) -> RcDoc<()> {
        let input = if self.input().is_tup() {
            self.input().to_doc().parens()
        } else {
            self.input().to_doc()
        };
        let output = if self.output().is_tup() {
            self.output().to_doc().parens()
        } else {
            self.output().to_doc()
        };
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
        RcDoc::text(self.id())
            .append(attr)
            .append(input)
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(output)
    }
}

impl PrettyPrint for Def {
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

impl PrettyPrint for Desc {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.def().iter().map(|(_, def)| def.to_doc()),
            RcDoc::hardline(),
        )
    }
}
