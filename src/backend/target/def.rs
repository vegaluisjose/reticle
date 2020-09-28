use crate::asm::ast::{InstrPrim, LocTy};
use crate::passes::select::tree::Tree;

#[derive(Clone, Debug)]
pub struct Tile {
    pub instr: InstrPrim,
    pub pattern: Tree,
    pub loc: LocTy,
    pub cost: f32,
}

#[derive(Clone, Debug)]
pub struct Descriptor {
    pub tiles: Vec<Tile>,
}
