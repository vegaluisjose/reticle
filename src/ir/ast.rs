use std::collections::HashMap;
use std::rc::Rc;

pub type Id = String;
pub type InstrMap = HashMap<Id, Instr>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Any,
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<Ty>, u64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExprTerm {
    Any,
    Val(i64),
    Var(Id, Ty),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ExprTup {
    pub term: Vec<ExprTerm>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Term(ExprTerm),
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
    Con,
    Sll,
    Srl,
    Sra,
    Ext,
    Cat,
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
pub struct OpCall {
    pub op: Id,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Def {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Prog {
    pub def: HashMap<Id, Def>,
}
