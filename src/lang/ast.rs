use serde::{Deserialize, Serialize};
use crate::util::pretty::{PrettyPrinter, PRETTY_INDENT};
use std::rc::Rc;
use pretty::RcDoc;
use std::fmt;

pub type Id = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataType {
    Placeholder,
    UInt(u64),
    SInt(i64),
    Vector(Rc<DataType>, u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlacedType {
    Lut,
    Lum,
    Dsp,
    Ram,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RelativeOp {
    Equal,
    Above,
    Below,
    Before,
    After,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Expr {
    Placeholder,
    ULit(u64),
    SLit(i64),
    VarRef(Id),
    Relative(RelativeOp, Rc<Expr>),
    Origin(PlacedType, Rc<Expr>, Rc<Expr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StdOp {
    Identity,
    ScalarConst,
    ScalarFromVec,
    ScalarSlice,
    ScalarExtract,
    ScalarCat,
    ScalarTruncate,
    ScalarExtend,
    VecConst,
    VecSlice,
    VecExtract,
    VecCat,
    VecFromScalar,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlacedOp {
    Reg,
    Add,
    Sub,
    Mul,
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Mux,
    Equal,
    Nequal,
    Gt,
    Lt,
    Ge,
    Le,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SimOp {
    Print {
        params: Vec<Expr>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompOp {
    Std {
        op: StdOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
    },
    Placed {
        op: PlacedOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
        loc: Expr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Port {
    Input {
        id: Id,
        datatype: DataType,
    },
    Output {
        id: Id,
        datatype: DataType,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Decl {
    Sim {
        op: SimOp,
    },
    Comp {
        op: CompOp,
        outputs: Vec<Port>,
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Def {
    Sim {
        name: Id,
        body: Vec<Decl>,
    },
    Comp {
        name: Id,
        inputs: Vec<Port>,
        outputs: Vec<Port>,
        body: Vec<Decl>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prog {
    pub defs: Vec<Def>,
}

// Pretty printer

impl PrettyPrinter for DataType {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            DataType::Placeholder => RcDoc::text("??"),
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

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for PlacedType {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            PlacedType::Lut => RcDoc::text("lut"),
            PlacedType::Lum => RcDoc::text("lum"),
            PlacedType::Dsp => RcDoc::text("dsp"),
            PlacedType::Ram => RcDoc::text("ram"),
        }
    }
}

impl fmt::Display for PlacedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for RelativeOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            RelativeOp::Equal => RcDoc::text("equal"),
            RelativeOp::Above => RcDoc::text("above"),
            RelativeOp::Below => RcDoc::text("below"),
            RelativeOp::Before => RcDoc::text("before"),
            RelativeOp::After => RcDoc::text("after"),
        }
    }
}

impl fmt::Display for RelativeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Placeholder => RcDoc::text("??"),
            Expr::ULit(n) => RcDoc::as_string(n),
            Expr::SLit(n) => RcDoc::as_string(n),
            Expr::VarRef(n) => RcDoc::as_string(n),
            Expr::Relative(op, n) => op.to_doc()
                .append(RcDoc::text("("))
                .append(RcDoc::as_string(n))
                .append(RcDoc::text(")")),
            Expr::Origin(ty, x, y) => ty.to_doc()
                .append(RcDoc::text("("))
                .append(x.to_doc())
                .append(RcDoc::text(","))
                .append(RcDoc::space())
                .append(y.to_doc())
                .append(RcDoc::text(")")),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for StdOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            _ => panic!("WIP"),
        }
    }
}

impl fmt::Display for StdOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for PlacedOp {
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

impl fmt::Display for PlacedOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for SimOp {
    fn to_doc(&self) -> RcDoc<()> {
        panic!("WIP")
    }
}

impl fmt::Display for SimOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}


impl PrettyPrinter for CompOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            CompOp::Placed { op, attrs, params, loc } => {
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

impl fmt::Display for CompOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input { id, datatype } =>  RcDoc::as_string(id)
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

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Sim { op } => op.to_doc(),
            Decl::Comp { op, outputs } => RcDoc::intersperse(
                outputs.iter().map(|o| o.to_doc()),
                RcDoc::text(",").append(RcDoc::space())
            )
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(op.to_doc()),

        }
    }
}


impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Def {
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
            Def::Comp { name, inputs, outputs, body } => {
                let inputs_doc = RcDoc::intersperse(
                    inputs.iter().map(|i| i.to_doc()),
                    RcDoc::text(",").append(RcDoc::space())
                );
                let outputs_doc = RcDoc::intersperse(
                    outputs.iter().map(|o| o.to_doc()),
                    RcDoc::text(",").append(RcDoc::space())
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


impl fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let mut defs_doc = RcDoc::nil();
        for d in self.defs.iter() {
            defs_doc = defs_doc.append(d.to_doc());
        }
        defs_doc
    }
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}
