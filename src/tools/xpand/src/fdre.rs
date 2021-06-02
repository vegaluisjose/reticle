use crate::errors::Error;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelReg, ExprCoord, Loc};
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::{inst_name_try_from_instr, vec_expr_try_from_expr};
use crate::{CLOCK, RESET};
use verilog::ast as vl;
use xir::ast as xir;

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
    pub input: Port,
    pub output: Port,
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

impl DefaultPort for Fdre {
    fn default_input_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("C".to_string(), 1);
        width.insert("R".to_string(), 1);
        width.insert("CE".to_string(), 1);
        width.insert("D".to_string(), 1);
        let mut connection = ConnectionMap::new();
        connection.insert("C".to_string(), vl::Expr::new_ref(CLOCK));
        connection.insert("R".to_string(), vl::Expr::new_ref(RESET));
        connection.insert("CE".to_string(), vl::Expr::new_ulit_hex(1, "0"));
        connection.insert("D".to_string(), vl::Expr::new_ulit_hex(1, "0"));
        Port { width, connection }
    }
    fn default_output_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("Q".to_string(), 1);
        let mut connection = ConnectionMap::new();
        connection.insert("Q".to_string(), vl::Expr::new_ref(""));
        Port { width, connection }
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
            input: Fdre::default_input_port(),
            output: Fdre::default_output_port(),
        }
    }
}

impl Fdre {
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
}

impl ToInstance for Fdre {
    fn to_instance(&self) -> vl::Instance {
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
    fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_input(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
        if let Some(p) = self.input.connection.get_mut(port) {
            *p = expr;
            Ok(())
        } else {
            let err = format!("input {} do not exist", port);
            Err(Error::new_xpand_error(&err))
        }
    }
    fn set_output(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
        if let Some(p) = self.output.connection.get_mut(port) {
            *p = expr;
            Ok(())
        } else {
            let err = format!("output {} do not exist", port);
            Err(Error::new_xpand_error(&err))
        }
    }
}

pub fn fdre_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut fdre = Fdre::default();
    let name = inst_name_try_from_instr(instr)?;
    fdre.set_name(&name);
    if let Some(loc) = instr.loc() {
        fdre.set_loc(loc.clone());
    }
    let input = ["D", "CE"];
    let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
    for (i, e) in input.iter().zip(arg) {
        fdre.set_input(i, e)?;
    }
    let output = ["Q"];
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    for (o, e) in output.iter().zip(dst) {
        fdre.set_output(o, e)?;
    }
    Ok(vec![fdre.to_stmt()])
}
