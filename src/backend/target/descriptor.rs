use crate::backend::asm::ast as asm;
use crate::backend::target::spec;
use crate::passes::select::instr as sel;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Tile {
    pub asm: asm::Instr,
    pub pattern: sel::Pattern,
}

#[derive(Clone, Debug)]
pub struct Descriptor {
    pub tiles: Vec<Tile>,
}

impl From<spec::Instr> for Tile {
    fn from(instr: spec::Instr) -> Self {
        Tile {
            asm: instr.to_asm_instr(),
            pattern: instr.to_pattern(),
        }
    }
}

impl From<spec::Spec> for Descriptor {
    fn from(spec: spec::Spec) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for instr in spec.isa.iter() {
            tiles.push(Tile::from(instr.clone()));
        }
        Descriptor {
            tiles: tiles.to_vec(),
        }
    }
}

impl FromStr for Descriptor {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let spec: spec::Spec = serde_json::from_str(input).expect("Error: parsing json");
        Ok(Descriptor::from(spec))
    }
}
