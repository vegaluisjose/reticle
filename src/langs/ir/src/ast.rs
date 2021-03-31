use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub type Id = String;
pub type InstrMap = HashMap<Id, Instr>;
pub type TermMap = HashMap<Id, ExprTerm>;
pub type ExprSet = HashSet<Expr>;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Ty {
    Any,
    Bool,
    UInt(u64),
    SInt(u64),
    Vector(Rc<Ty>, u64),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum ExprTerm {
    Any,
    Val(i64),
    Var(Id, Ty),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash, Default)]
pub struct ExprTup {
    pub term: Vec<ExprTerm>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Expr {
    Term(ExprTerm),
    Tup(ExprTup),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Prim {
    Any,
    Lut,
    Dsp,
    Lram,
    Bram,
    Uram,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum OpWire {
    Id,
    Con,
    Sll,
    Srl,
    Sra,
    Ext,
    Cat,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum OpPrim {
    Reg,
    Ram,
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct OpCall {
    pub op: Id,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct InstrWire {
    pub op: OpWire,
    pub dst: Expr,
    pub attr: Expr,
    pub arg: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct InstrPrim {
    pub op: OpPrim,
    pub dst: Expr,
    pub attr: Expr,
    pub arg: Expr,
    pub prim: Prim,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct InstrCall {
    pub op: OpCall,
    pub dst: Expr,
    pub arg: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Prim(InstrPrim),
    Call(InstrCall),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct Sig {
    pub id: Id,
    pub input: Expr,
    pub output: Expr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash, Default)]
pub struct Def {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Default)]
pub struct Prog {
    pub def: HashMap<Id, Def>,
}
