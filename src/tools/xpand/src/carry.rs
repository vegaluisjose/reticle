use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelCarry, ExprCoord, Loc};
use crate::port::{Input, Output};
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub enum CarryType {
    Dual,
    Single,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub ty: CarryType,
}

#[derive(Clone, Debug)]
pub struct Carry {
    pub name: String,
    pub prim: String,
    pub attr: Attr,
    pub loc: Loc,
    pub input: Input,
    pub output: Output,
}

impl Default for Attr {
    fn default() -> Self {
        Attr {
            ty: CarryType::Single,
        }
    }
}

impl Default for Carry {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Carry(BelCarry::Carry8),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Carry {
            name: String::new(),
            prim: "CARRY8".to_string(),
            loc,
            attr: Attr::default(),
            input: Input::carry(),
            output: Output::carry(),
        }
    }
}

impl Carry {
    pub fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        match self.attr.ty {
            CarryType::Single => inst.add_param("CARRY_TYPE", vl::Expr::new_str("SINGLE_CY8")),
            CarryType::Dual => inst.add_param("CARRY_TYPE", vl::Expr::new_str("DUAL_CY4")),
        }
        for (k, v) in self.input.connection.iter() {
            inst.connect(&k, v.clone());
        }
        for (k, v) in self.output.connection.iter() {
            inst.connect(&k, v.clone());
        }
        if self.loc.is_placed() {
            let attr = attr_from_loc(&self.loc);
            inst.set_attr(attr);
        }
        inst
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
