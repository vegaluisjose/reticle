use crate::mdl::ast::*;
// use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::util::pretty_print::PrettyPrint;
// use itertools::Itertools;
use pretty::RcDoc;

impl PrettyPrint for OpReg {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            OpReg::Fdre => RcDoc::text("fdre"),
            OpReg::Fdse => RcDoc::text("fdse"),
        }
    }
}
