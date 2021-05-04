use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelLut, ExprCoord, Loc};
use crate::port::{Input, Output};
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub struct Attr {
    pub init: u64,
}

#[derive(Clone, Debug)]
pub struct Lut2 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

impl Default for Attr {
    fn default() -> Self {
        Attr { init: 0 }
    }
}

impl Default for Lut2 {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Lut(BelLut::A6),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Lut2 {
            name: String::new(),
            prim: "LUT2".to_string(),
            loc,
            attr: Attr::default(),
            input: Input::lut2(),
            output: Output::lut(),
        }
    }
}

impl Lut2 {
    pub fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        let init = format!("{:X}", self.attr.init);
        inst.add_param("INIT", vl::Expr::new_ulit_hex(4, &init));
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
