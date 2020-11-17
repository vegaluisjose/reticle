use crate::asm::ast::{InstrPhy, Prim};
use crate::backend::target::spec::*;
use crate::backend::target::{Descriptor, Tile};
use crate::passes::select::tree::Tree;

impl Tile {
    pub fn instr(&self) -> &InstrPhy {
        &self.instr
    }

    pub fn pattern(&self) -> &Tree {
        &self.pattern
    }

    pub fn loc(&self) -> &Prim {
        &self.loc
    }

    pub fn cost(&self) -> f32 {
        self.cost
    }

    pub fn set_instr(&mut self, instr: InstrPhy) {
        self.instr = instr;
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

    pub fn loc(&self) -> String {
        self.loc.to_string()
    }

    pub fn expr(&self) -> &SpecExpr {
        &self.expr
    }

    pub fn ty(&self) -> String {
        self.expr().ty()
    }
}

impl SpecExpr {
    pub fn ty(&self) -> String {
        match self {
            SpecExpr::Input(s) => s.to_string(),
            SpecExpr::UnOp(s, _, _) => s.to_string(),
            SpecExpr::BinOp(s, _, _, _) => s.to_string(),
            SpecExpr::TerOp(s, _, _, _, _) => s.to_string(),
        }
    }
}
