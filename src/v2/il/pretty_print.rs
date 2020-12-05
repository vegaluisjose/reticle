use crate::v2::il::ast::*;
// use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::util::pretty_print::{intersperse, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for Sig {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text(self.id())
    }
}

impl PrettyPrint for Def {
    fn to_doc(&self) -> RcDoc<()> {
        self.sig().to_doc()
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.defs().iter().map(|(_, def)| def.to_doc()),
            RcDoc::hardline(),
        )
    }
}
