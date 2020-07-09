use crate::util::pretty::{PrettyPrinter, PRETTY_INDENT};
use pretty::RcDoc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

pub type Id = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataType {
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<DataType>, u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Expr {
    Ref(Id),
    ULit(u64),
    SLit(i64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LocType {
    Lut,
    Lum,
    Dsp,
    Ram,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LocOp {
    Equal,
    Above,
    Below,
    Before,
    After,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Loc {
    Unknown,
    Relative(LocOp, Id),
    Origin(LocType, u64, u64),
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
pub enum Dop {
    Print { params: Vec<Expr> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Op {
    Std {
        op: StdOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
    },
    Placed {
        op: PlacedOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
        loc: Loc,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Port {
    Input { id: Id, datatype: DataType },
    Output { id: Id, datatype: DataType },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Decl {
    Debug { op: Dop }, // need to find a better name for this
    Instr { op: Op, outputs: Vec<Port> },
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

impl PrettyPrinter for DataType {
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

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for LocType {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            LocType::Lut => RcDoc::text("lut"),
            LocType::Lum => RcDoc::text("lum"),
            LocType::Dsp => RcDoc::text("dsp"),
            LocType::Ram => RcDoc::text("ram"),
        }
    }
}

impl fmt::Display for LocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for LocOp {
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

impl fmt::Display for LocOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Loc::Unknown => RcDoc::text("??"),
            Loc::Relative(op, n) => op
                .to_doc()
                .append(RcDoc::text("("))
                .append(RcDoc::as_string(n))
                .append(RcDoc::text(")")),
            Loc::Origin(ty, x, y) => ty
                .to_doc()
                .append(RcDoc::text("("))
                .append(RcDoc::as_string(x))
                .append(RcDoc::text(","))
                .append(RcDoc::space())
                .append(RcDoc::as_string(y))
                .append(RcDoc::text(")")),
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::ULit(n) => RcDoc::as_string(n),
            Expr::SLit(n) => RcDoc::as_string(n),
            Expr::Ref(n) => RcDoc::as_string(n),
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

impl PrettyPrinter for Dop {
    fn to_doc(&self) -> RcDoc<()> {
        panic!("WIP")
    }
}

impl fmt::Display for Dop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Op {
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

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Port {
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

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl PrettyPrinter for Decl {
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

// helper functions

impl Expr {
    pub fn get_id(self) -> Id {
        match self {
            Expr::Ref(name) => name.to_string(),
            _ => panic!("Error: expr is not Ref"),
        }
    }
}

impl FromStr for DataType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re_bool = Regex::new(r"bool$").unwrap();
        let re_uint = Regex::new(r"^u([[:alnum:]]+)$").unwrap();
        let re_sint = Regex::new(r"^i([[:alnum:]]+)$").unwrap();
        let re_uvec = Regex::new(r"^u([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let re_svec = Regex::new(r"^i([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let mut dtype = Err(());
        let caps;
        if re_bool.is_match(input) {
            dtype = Ok(DataType::Bool);
        } else if re_uint.is_match(input) {
            caps = re_uint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 0, "Error: width must be greater than zero");
                dtype = Ok(DataType::UInt(width));
            }
        } else if re_sint.is_match(input) {
            caps = re_sint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 1, "Error: width must be greater than one");
                dtype = Ok(DataType::SInt(width));
            }
        } else if re_uvec.is_match(input) {
            caps = re_uvec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 0, "Error: width must be greater than zero");
                    assert!(len > 0, "Error: length must be greater than zero");
                    dtype = Ok(DataType::Vector(Rc::new(DataType::UInt(width)), len));
                }
            }
        } else if re_svec.is_match(input) {
            caps = re_svec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 1, "Error: width must be greater than one");
                    assert!(len > 0, "Error: length must be greater than zero");
                    dtype = Ok(DataType::Vector(Rc::new(DataType::SInt(width)), len));
                }
            }
        }
        dtype
    }
}

impl FromStr for PlacedOp {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.as_ref() {
            "reg" => Ok(PlacedOp::Reg),
            "add" => Ok(PlacedOp::Add),
            "sub" => Ok(PlacedOp::Sub),
            "mul" => Ok(PlacedOp::Mul),
            _  => panic!("WIP"),
        }
    }
}

impl Op {
    pub fn get_params(&self) -> &Vec<Expr> {
        match self {
            Op::Std {
                op: _,
                attrs: _,
                params,
            } => params,
            Op::Placed {
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params,
        }
    }
}

impl Decl {
    pub fn new_instr(dst: &str, ty: &str, op: &str, lhs: &str, rhs: &str) -> Decl {
        let placed_op = Op::Placed {
            op: PlacedOp::from_str(op).unwrap(),
            attrs: vec![],
            params: vec![
                Expr::Ref(lhs.to_string()),
                Expr::Ref(rhs.to_string()),
            ],
            loc: Loc::Unknown,
        };
        let output = Port::Output {
            id: dst.to_string(),
            datatype: DataType::from_str(ty).unwrap(),
        };
        Decl::Instr {
            op: placed_op,
            outputs: vec![output],
        }
    }
}

impl Def {
    pub fn new_comp(name: &str) -> Def {
        Def::Comp {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_input(&mut self, name: &str, ty: &str) {
        match self {
            Def::Comp {
                name: _,
                inputs,
                outputs: _,
                body: _,
            } => {
                let dtype = DataType::from_str(ty).unwrap();
                let port = Port::Input {
                    id: name.to_string(),
                    datatype: dtype,
                };
                inputs.push(port);
            }
            _ => panic!("Error: sim definition does not support inputs"),
        }
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        match self {
            Def::Comp {
                name: _,
                inputs: _,
                outputs,
                body: _,
            } => {
                let dtype = DataType::from_str(ty).unwrap();
                let port = Port::Output {
                    id: name.to_string(),
                    datatype: dtype,
                };
                outputs.push(port);
            }
            _ => panic!("Error: sim definition does not support outputs"),
        }
    }

    pub fn add_decl(&mut self, decl: Decl) {
        match self {
            Def::Comp {
                name: _,
                inputs: _,
                outputs: _,
                body,
            } => {
                body.push(decl);
            }
            Def::Sim {
                name: _,
                body,
            } => {
                body.push(decl);
            }
        }
    }
}

impl Prog {
    pub fn new() -> Prog {
        Prog { defs: Vec::new() }
    }

    pub fn add_def(&mut self, def: Def) {
        self.defs.push(def);
    }
}
