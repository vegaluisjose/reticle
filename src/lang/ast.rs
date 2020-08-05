use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub type Id = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Hole,
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<Ty>, u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Expr {
    Int(i64),
    Ref(Id, Ty),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Loc {
    Hole,
    Lut,
    Dsp,
    Lum,
    Ram,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StdOp {
    Identity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PrimOp {
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
    NotEqual,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Instr {
    Std {
        id: Id,
        ty: Ty,
        op: StdOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
    },
    Prim {
        id: Id,
        ty: Ty,
        op: PrimOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
        loc: Loc,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Port {
    Input { id: Id, ty: Ty },
    Output { id: Id, ty: Ty },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sig {
    pub id: Id,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Def {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prog {
    pub defs: Vec<Def>,
}
