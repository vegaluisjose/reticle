use crate::util::pretty_print::{block_with_braces, intersperse, PrettyHelper, PrettyPrint};
use crate::v2::il::ast::*;
use itertools::Itertools;
use pretty::RcDoc;

fn expr_names(expr: &Expr) -> RcDoc<()> {
    match expr {
        Expr::Val(v) => RcDoc::as_string(v),
        Expr::Name(n, _) => RcDoc::as_string(n),
        Expr::Tup(tup) if tup.is_empty() => RcDoc::nil(),
        Expr::Tup(tup) => intersperse(
            tup.expr().iter().map(|n| expr_names(n)),
            RcDoc::text(",").append(RcDoc::space()),
        )
        .parens(),
    }
}

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Var => RcDoc::text("??"),
            Ty::Bool => RcDoc::text("bool"),
            Ty::UInt(width) => RcDoc::text("u").append(RcDoc::as_string(width)),
            Ty::SInt(width) => RcDoc::text("i").append(RcDoc::as_string(width)),
            Ty::Vector(dtype, len) => dtype.to_doc().append(RcDoc::as_string(len)).comps(),
        }
    }
}

impl PrettyPrint for ExprTup {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.expr().iter().map(|e| e.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        )
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Val(v) => RcDoc::as_string(v),
            Expr::Name(n, ty) => RcDoc::as_string(n)
                .append(RcDoc::text(":"))
                .append(ty.to_doc()),
            Expr::Tup(tup) => tup.to_doc(),
        }
    }
}

impl PrettyPrint for CallOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            CallOp::Op(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for WireOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            WireOp::Id => RcDoc::text("id"),
            WireOp::Con => RcDoc::text("const"),
            WireOp::Sll => RcDoc::text("sll"),
            WireOp::Srl => RcDoc::text("srl"),
            WireOp::Sra => RcDoc::text("sra"),
        }
    }
}

impl PrettyPrint for CompOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            CompOp::Reg => RcDoc::text("reg"),
            CompOp::Add => RcDoc::text("add"),
            CompOp::Sub => RcDoc::text("sub"),
            CompOp::Mul => RcDoc::text("mul"),
            CompOp::Not => RcDoc::text("not"),
            CompOp::And => RcDoc::text("and"),
            CompOp::Or => RcDoc::text("or"),
            CompOp::Xor => RcDoc::text("xor"),
            CompOp::Mux => RcDoc::text("mux"),
            CompOp::Eql => RcDoc::text("eq"),
            CompOp::Neql => RcDoc::text("neq"),
            CompOp::Gt => RcDoc::text("gt"),
            CompOp::Lt => RcDoc::text("lt"),
            CompOp::Ge => RcDoc::text("ge"),
            CompOp::Le => RcDoc::text("le"),
        }
    }
}

impl PrettyPrint for InstrCall {
    fn to_doc(&self) -> RcDoc<()> {
        self.dst()
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(self.op().to_doc())
            .append(expr_names(self.arg()))
    }
}

impl PrettyPrint for InstrWire {
    fn to_doc(&self) -> RcDoc<()> {
        let attr = if self.attr().is_tup() && self.attr().tup().is_empty() {
            RcDoc::nil()
        } else {
            self.attr().to_doc().brackets()
        };
        self.dst()
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(self.op().to_doc())
            .append(attr)
            .append(expr_names(self.arg()))
    }
}

impl PrettyPrint for InstrComp {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::nil()
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Call(instr) => instr.to_doc(),
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
        RcDoc::text(self.id())
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

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let main = self.get("main");
        if self.def().len() == 1 {
            main.to_doc()
        } else {
            intersperse(
                self.def()
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
