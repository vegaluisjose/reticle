use crate::backend::asm::ast::Instr;
use crate::backend::target::spec::*;
use crate::backend::target::{Descriptor, Tile};
use crate::passes::map::tree::Tree;

impl Tile {
    pub fn instr(&self) -> &Instr {
        &self.instr
    }

    pub fn pattern(&self) -> &Tree {
        &self.pattern
    }
}

impl Descriptor {
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

impl SpecCost {
    pub fn delay(&self) -> u32 {
        self.delay
    }
    pub fn area(&self) -> u32 {
        self.area
    }
}

impl SpecInstr {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn delay(&self) -> u32 {
        self.cost.delay()
    }

    pub fn area(&self) -> u32 {
        self.cost.area()
    }

    pub fn ty(&self) -> String {
        self.ty.to_string()
    }

    pub fn loc(&self) -> String {
        self.loc.to_string()
    }

    pub fn expr(&self) -> &SpecExpr {
        &self.expr
    }
}
