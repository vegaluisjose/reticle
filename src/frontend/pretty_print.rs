use crate::lang::ast::*;
use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Hole => RcDoc::text("??"),
            Ty::Bool => RcDoc::text("bool"),
            Ty::UInt(width) => RcDoc::text("u").append(RcDoc::as_string(width)),
            Ty::SInt(width) => RcDoc::text("i").append(RcDoc::as_string(width)),
            Ty::Vector(dtype, len) => dtype
                .to_doc()
                .append(RcDoc::text("<"))
                .append(RcDoc::as_string(len))
                .append(RcDoc::text(">")),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Loc::Hole => RcDoc::text("??"),
            Loc::Lut => RcDoc::text("lut"),
            Loc::Lum => RcDoc::text("lum"),
            Loc::Dsp => RcDoc::text("dsp"),
            Loc::Ram => RcDoc::text("ram"),
        }
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(n, _) => RcDoc::as_string(n),
            Expr::Int(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for StdOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            StdOp::Identity => RcDoc::text("id"),
            StdOp::Const => RcDoc::text("const"),
            StdOp::ShiftLeft => RcDoc::text("shl"),
            StdOp::ShiftRight => RcDoc::text("shr"),
        }
    }
}

impl PrettyPrint for PrimOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            PrimOp::Reg => RcDoc::text("reg"),
            PrimOp::Add => RcDoc::text("add"),
            PrimOp::Sub => RcDoc::text("sub"),
            PrimOp::Mul => RcDoc::text("mul"),
            PrimOp::Not => RcDoc::text("not"),
            PrimOp::And => RcDoc::text("and"),
            PrimOp::Nand => RcDoc::text("nand"),
            PrimOp::Or => RcDoc::text("or"),
            PrimOp::Nor => RcDoc::text("nor"),
            PrimOp::Xor => RcDoc::text("xor"),
            PrimOp::Xnor => RcDoc::text("xnor"),
            PrimOp::Mux => RcDoc::text("mux"),
            PrimOp::Equal => RcDoc::text("eq"),
            PrimOp::NotEqual => RcDoc::text("neq"),
            PrimOp::Gt => RcDoc::text("gt"),
            PrimOp::Lt => RcDoc::text("lt"),
            PrimOp::Ge => RcDoc::text("ge"),
            PrimOp::Le => RcDoc::text("le"),
        }
    }
}

impl PrettyPrint for InstrStd {
    fn to_doc(&self) -> RcDoc<()> {
        let id = if self.dst_id().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::as_string(&self.dst_id())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(self.dst_ty().to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
        };
        let attrs = if self.attrs().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.attrs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .brackets()
        };
        let params = if self.params().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.params().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        id.append(self.op.to_doc()).append(attrs).append(params)
    }
}

impl PrettyPrint for InstrPrim {
    fn to_doc(&self) -> RcDoc<()> {
        let id = if self.dst_id().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::as_string(&self.dst_id())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(self.dst_ty().to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
        };
        let attrs = if self.attrs().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.attrs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .brackets()
        };
        let params = if self.params().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.params().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        id.append(self.op().to_doc())
            .append(attrs)
            .append(params)
            .append(RcDoc::space())
            .append(self.loc().to_doc())
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Std(instr) => instr.to_doc(),
            Instr::Prim(instr) => instr.to_doc(),
        }
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(&self.id())
            .append(RcDoc::text(":"))
            .append(RcDoc::space())
            .append(self.ty().to_doc())
    }
}

impl PrettyPrint for Def {
    fn to_doc(&self) -> RcDoc<()> {
        let inputs = if self.inputs().is_empty() {
            RcDoc::nil().parens()
        } else {
            intersperse(
                self.inputs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        let outputs = if self.outputs().is_empty() {
            RcDoc::nil().parens()
        } else {
            intersperse(
                self.outputs().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .parens()
        };
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.body()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        let name = RcDoc::text("def")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.id()))
            .append(inputs)
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(outputs);
        block_with_braces(name, body)
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let mut defs_doc = RcDoc::nil();
        for d in self.defs().iter() {
            defs_doc = defs_doc.append(d.to_doc());
        }
        defs_doc
    }
}
