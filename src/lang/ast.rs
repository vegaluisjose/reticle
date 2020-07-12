use serde::{Deserialize, Serialize};
use std::rc::Rc;

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
pub enum Loc {
    Unknown,
    Lut,
    Lum,
    Dsp,
    Ram,
    Ref(Id),
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
