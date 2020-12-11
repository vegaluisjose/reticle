use std::collections::HashMap;
use std::rc::Rc;

pub type Id = String;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Any,
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<Ty>, u64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ExprTup {
    pub expr: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Val(i64),
    Var(Id, Ty),
    Tup(ExprTup),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prim {
    Any,
    Lut,
    Dsp,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpWire {
    Id,
    Inp,
    Con,
    Sll,
    Srl,
    Sra,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpComp {
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
pub enum OpCall {
    Op(Id),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrWire {
    pub op: OpWire,
    pub dst: Expr,
    pub attr: Expr,
    pub arg: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrComp {
    pub op: OpComp,
    pub dst: Expr,
    pub attr: Expr,
    pub arg: Expr,
    pub prim: Prim,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrCall {
    pub op: OpCall,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Comp(InstrComp),
    Call(InstrCall),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sig {
    pub id: Id,
    pub input: Expr,
    pub output: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Def {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Prog {
    pub def: HashMap<Id, Def>,
}
