use crate::lang::ast;

pub type Id = ast::Id;
pub type Ty = ast::Ty;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    Any,
    In,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Loc {
    Any,
    Hole,
    Lut,
    Lum,
    Dsp,
    Ram,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Instr {
    pub loc: Loc,
    pub ty: Ty,
    pub op: Op,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub name: String,
    pub cost: u32,
    pub instr: Vec<Instr>,
}
