use std::collections::HashMap;
use std::rc::Rc;

pub type Id = String;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Var,
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<Ty>, u64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ExprTup {
    pub exprs: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Val(i64),
    Name(Id, Ty),
    Tup(ExprTup),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prim {
    Var,
    Lut,
    Dsp,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WireOp {
    Id,
    Con,
    Sll,
    Srl,
    Sra,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompOp {
    Reg,
    Add,
    Sub,
    Mul,
    Not,
    And,
    Or,
    Xor,
    Mux,
    Eql,
    Neql,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallOp {
    Op(Id),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrWire {
    pub op: WireOp,
    pub dst: Expr,
    pub attrs: Expr,
    pub args: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrComp {
    pub op: CompOp,
    pub dst: Expr,
    pub attrs: Expr,
    pub args: Expr,
    pub prim: Prim,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrCall {
    pub op: CallOp,
    pub dst: Expr,
    pub args: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Comp(InstrComp),
    Call(InstrCall),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Signature {
    pub id: Id,
    pub inputs: Expr,
    pub outputs: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Def {
    pub sig: Signature,
    pub body: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prog {
    pub def_map: HashMap<Id, Def>,
}
