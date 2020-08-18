use crate::backend::asm::ast::Instr;

#[derive(Clone, Debug)]
pub struct Tile {
    pub instr: Instr,
}

#[derive(Clone, Debug)]
pub struct Descriptor {
    pub tiles: Vec<Tile>,
}
