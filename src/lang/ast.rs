use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub type Id = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<Ty>, u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Expr {
    Ref(Id, Ty),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Loc {
    Var,
    Lut,
    Lum,
    Dsp,
    Ram,
    Ref(Id),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StdOp {
    Identity,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Port {
    Input { id: Id, ty: Ty },
    Output { id: Id, ty: Ty },
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
    Placed {
        id: Id,
        ty: Ty,
        op: PlacedOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
        loc: Loc,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sig {
    pub name: Id,
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
