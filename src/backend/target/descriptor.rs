use crate::backend::asm::ast::Instr;
use crate::passes::map::partition::tree::Tree;

#[derive(Clone, Debug)]
pub struct Tile {
    pub instr: Instr,
    pub pattern: Tree,
}

#[derive(Clone, Debug)]
pub struct Descriptor {
    pub tiles: Vec<Tile>,
}
