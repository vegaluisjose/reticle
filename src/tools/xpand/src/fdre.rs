use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelReg, ExprCoord, Loc};
use crate::port::{Input, Output};
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub struct Attr {
    pub init: bool,
    pub is_c_inverted: bool,
    pub is_d_inverted: bool,
    pub is_r_inverted: bool,
}

#[derive(Clone, Debug)]
pub struct Fdre {
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
            init: false,
            is_c_inverted: false,
            is_d_inverted: false,
            is_r_inverted: false,
        }
    }
}

impl Default for Fdre {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Reg(BelReg::A),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Fdre {
            name: String::new(),
            prim: "FDRE".to_string(),
            loc,
            attr: Attr::default(),
            input: Input::fdre(),
            output: Output::fdre(),
        }
    }
}


impl Fdre {
    pub fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        let init = format!("{}", u32::from(self.attr.init));
        let is_c_inv = format!("{}", u32::from(self.attr.is_c_inverted));
        let is_d_inv = format!("{}", u32::from(self.attr.is_d_inverted));
        let is_r_inv = format!("{}", u32::from(self.attr.is_r_inverted));
        inst.add_param("INIT", vl::Expr::new_ulit_bin(1, &init));
        inst.add_param("IS_C_INVERTED", vl::Expr::new_ulit_bin(1, &is_c_inv));
        inst.add_param("IS_D_INVERTED", vl::Expr::new_ulit_bin(1, &is_d_inv));
        inst.add_param("IS_R_INVERTED", vl::Expr::new_ulit_bin(1, &is_r_inv));
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
