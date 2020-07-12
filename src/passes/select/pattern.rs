use crate::passes::select::instr::*;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub name: String,
    pub seq: Vec<Instr>,
    pub cost: i32,
}
