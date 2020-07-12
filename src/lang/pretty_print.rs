use crate::lang::ast::*;
use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
use pretty::RcDoc;

impl PrettyPrint for DataType {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            DataType::Bool => RcDoc::text("bool"),
            DataType::UInt(width) => RcDoc::text("u").append(RcDoc::as_string(width)),
            DataType::SInt(width) => RcDoc::text("i").append(RcDoc::as_string(width)),
            DataType::Vector(dtype, len) => dtype
                .to_doc()
                .append(RcDoc::text("<"))
                .append(RcDoc::as_string(len))
                .append(RcDoc::text(">")),
        }
    }
}

impl PrettyPrint for LocType {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            LocType::Lut => RcDoc::text("lut"),
            LocType::Lum => RcDoc::text("lum"),
            LocType::Dsp => RcDoc::text("dsp"),
            LocType::Ram => RcDoc::text("ram"),
        }
    }
}

impl PrettyPrint for LocOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            LocOp::Equal => RcDoc::text("equal"),
            LocOp::Above => RcDoc::text("above"),
            LocOp::Below => RcDoc::text("below"),
            LocOp::Before => RcDoc::text("before"),
            LocOp::After => RcDoc::text("after"),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Loc::Unknown => RcDoc::text("??"),
            Loc::Place(ty) => ty
                .to_doc()
                .append(RcDoc::text("("))
                .append(RcDoc::text("??"))
                .append(RcDoc::text(","))
                .append(RcDoc::space())
                .append(RcDoc::text("??"))
                .append(RcDoc::text(")")),
            Loc::Origin(ty, x, y) => ty
                .to_doc()
                .append(RcDoc::text("("))
                .append(RcDoc::as_string(x))
                .append(RcDoc::text(","))
                .append(RcDoc::space())
                .append(RcDoc::as_string(y))
                .append(RcDoc::text(")")),
            Loc::Relative(op, n) => op
                .to_doc()
                .append(RcDoc::text("("))
                .append(RcDoc::as_string(n))
                .append(RcDoc::text(")")),
        }
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::ULit(n) => RcDoc::as_string(n),
            Expr::SLit(n) => RcDoc::as_string(n),
            Expr::Ref(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for StdOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            _ => panic!("WIP"),
        }
    }
}

impl PrettyPrint for PlacedOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            PlacedOp::Reg => RcDoc::text("reg"),
            PlacedOp::Add => RcDoc::text("add"),
            PlacedOp::Sub => RcDoc::text("sub"),
            PlacedOp::Mul => RcDoc::text("mul"),
            _ => panic!("WIP"),
        }
    }
}

impl PrettyPrint for Dop {
    fn to_doc(&self) -> RcDoc<()> {
        panic!("WIP")
    }
}

impl PrettyPrint for Op {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Op::Placed {
                op,
                attrs,
                params,
                loc,
            } => {
                let attrs_doc = match attrs.is_empty() {
                    true => RcDoc::nil(),
                    false => RcDoc::text("[")
                        .append(RcDoc::intersperse(
                            attrs.iter().map(|a| a.to_doc()),
                            RcDoc::text(",").append(RcDoc::space()),
                        ))
                        .append(RcDoc::text("]")),
                };
                let params_doc = match params.is_empty() {
                    true => panic!("Error: must have at least one param"),
                    false => RcDoc::text("(")
                        .append(RcDoc::intersperse(
                            params.iter().map(|p| p.to_doc()),
                            RcDoc::text(",").append(RcDoc::space()),
                        ))
                        .append(RcDoc::text(")")),
                };
                let loc_doc = RcDoc::text("@").append(loc.to_doc());
                op.to_doc()
                    .append(attrs_doc)
                    .append(params_doc)
                    .append(RcDoc::space())
                    .append(loc_doc)
            }
            _ => panic!("WIP"),
        }
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input { id, datatype } => RcDoc::as_string(id)
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(datatype.to_doc()),
            Port::Output { id, datatype } => RcDoc::as_string(id)
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(datatype.to_doc()),
        }
    }
}

impl PrettyPrint for Def {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Def::Sim { name, body } => {
                let mut body_doc = RcDoc::nil();
                for decl in body.iter() {
                    body_doc = body_doc
                        .append(RcDoc::hardline())
                        .append(decl.to_doc())
                        .append(RcDoc::text(";"));
                }
                body_doc = body_doc.nest(PRETTY_INDENT).group();
                RcDoc::text("sim")
                    .append(RcDoc::space())
                    .append(RcDoc::as_string(name))
                    .append(RcDoc::text("{"))
                    .append(body_doc)
                    .append(RcDoc::text("}"))
            }
            Def::Comp {
                name,
                inputs,
                outputs,
                body,
            } => {
                let inputs_doc = RcDoc::intersperse(
                    inputs.iter().map(|i| i.to_doc()),
                    RcDoc::text(",").append(RcDoc::space()),
                );
                let outputs_doc = RcDoc::intersperse(
                    outputs.iter().map(|o| o.to_doc()),
                    RcDoc::text(",").append(RcDoc::space()),
                );
                let mut body_doc = RcDoc::nil();
                for decl in body.iter() {
                    body_doc = body_doc
                        .append(RcDoc::hardline())
                        .append(decl.to_doc())
                        .append(RcDoc::text(";"));
                }
                body_doc = body_doc.nest(PRETTY_INDENT).group();
                RcDoc::text("def")
                    .append(RcDoc::space())
                    .append(RcDoc::as_string(name))
                    .append(RcDoc::text("("))
                    .append(inputs_doc)
                    .append(RcDoc::text(")"))
                    .append(RcDoc::space())
                    .append(RcDoc::text("->"))
                    .append(RcDoc::space())
                    .append(RcDoc::text("("))
                    .append(outputs_doc)
                    .append(RcDoc::text(")"))
                    .append(RcDoc::space())
                    .append(RcDoc::text("{"))
                    .append(body_doc)
                    .append(RcDoc::hardline())
                    .append(RcDoc::text("}"))
            }
        }
    }
}

impl PrettyPrint for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Debug { op } => op.to_doc(),
            Decl::Instr { op, outputs } => RcDoc::intersperse(
                outputs.iter().map(|o| o.to_doc()),
                RcDoc::text(",").append(RcDoc::space()),
            )
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(op.to_doc()),
        }
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let mut defs_doc = RcDoc::nil();
        for d in self.defs.iter() {
            defs_doc = defs_doc.append(d.to_doc());
        }
        defs_doc
    }
}
