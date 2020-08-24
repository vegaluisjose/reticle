use crate::backend::asm::ast::{Instr, LocTy};
use crate::passes::map::tree::Tree;

#[derive(Clone, Debug)]
pub struct Tile {
    pub instr: Instr,
    pub pattern: Tree,
    pub loc: LocTy,
}

#[derive(Clone, Debug)]
pub struct Descriptor {
    pub tiles: Vec<Tile>,
}
