use crate::v2::il::ast::*;
// use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::util::pretty_print::{intersperse, PrettyPrint};
use itertools::Itertools;
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
