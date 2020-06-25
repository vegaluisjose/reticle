use serde::{Deserialize, Serialize};
//use crate::util::pretty::PrettyPrinter;
use std::rc::Rc;

pub type Id = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataType {
    Placeholder,
    UInt(u64),
    SInt(i64),
    Vector(Rc<DataType>, u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LocType {
    Lut,
    Lum,
    Dsp,
    Ram,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Expr {
    Placeholder,
    ULit(u64),
    SLit(i64),
    VarRef(Id),
    LocRef(Id),
    Loc(LocType, Rc<Expr>, Rc<Expr>),
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
pub enum Prim {
    Std {
        op: StdOp,
        attr: Vec<Expr>,
        params: Vec<Expr>,
    },
    Placed {
        op: PlacedOp,
        attr: Vec<Expr>,
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
pub struct Decl {
    pub prim: Prim,
    pub outputs: Vec<Port>,
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
