use crate::asm::ast::{InstrPhy, Prim};
use crate::passes::select::tree::Tree;

#[derive(Clone, Debug)]
pub struct Tile {
    pub instr: InstrPhy,
    pub pattern: Tree,
    pub loc: Prim,
    pub cost: f32,
}

#[derive(Clone, Debug)]
pub struct Descriptor {
    pub tiles: Vec<Tile>,
}
