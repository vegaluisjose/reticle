use crate::errors::Error;
use crate::inst_name_try_from_instr;
use crate::loc::Loc;
use crate::to_verilog::{ToVerilogExpr, ToVerilogInstance, VerilogExprMap};
use prim::ultrascale::clock::CLOCK;
use prim::ultrascale::gnd::GND;
use prim::ultrascale::lram::{Lram, ParamValue};
use prim::{ParamSet, PortSet};
use verilog::ast as vl;
use xir::ast::InstrMach;

impl ToVerilogExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::Bool(value) => {
                let num = format!("{}", *value as u32);
                vl::Expr::new_ulit_bin(1, &num)
            }
            ParamValue::Bytes(width, values) => {
                let mut num = String::new();
                for v in values.iter().rev() {
                    let val = format!("{:02X}", v);
                    num.push_str(&val);
                }
                vl::Expr::new_ulit_hex(*width, &num)
            }
        }
    }
}

impl ToVerilogInstance<ParamValue> for Lram {
    fn to_name(&self) -> String {
        String::new()
    }
    fn to_prim(&self) -> String {
        self.name()
    }
    fn to_param_set(&self) -> &ParamSet<ParamValue> {
        self.param()
    }
    fn to_input_set(&self) -> &PortSet {
        self.input()
    }
    fn to_output_set(&self) -> &PortSet {
        self.output()
    }
}

#[derive(Clone, Debug)]
struct Rom {
    pub prim: Lram,
    pub instr: InstrMach,
}

impl Rom {
    pub fn new(instr: InstrMach) -> Self {
        Rom {
            prim: Lram::default(),
            instr,
        }
    }
    pub fn instr(&self) -> &InstrMach {
        &self.instr
    }
}

impl ToVerilogInstance<ParamValue> for Rom {
    fn to_name(&self) -> String {
        inst_name_try_from_instr(&self.instr).unwrap()
    }
    fn to_prim(&self) -> String {
        self.prim.name()
    }
    fn to_param_set(&self) -> &ParamSet<ParamValue> {
        self.prim.param()
    }
    fn to_input_set(&self) -> &PortSet {
        self.prim.input()
    }
    fn to_output_set(&self) -> &PortSet {
        self.prim.output()
    }
    fn to_param_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        if let Some(mem) = self.instr().mem() {
            for p in self.to_param_set().iter() {
                let mut value: Vec<u8> = Vec::new();
                for v in mem.values().iter() {
                    match p.name().as_str() {
                        "INIT_A" => value.push(v & 1),
                        "INIT_B" => value.push((v >> 1) & 1),
                        "INIT_C" => value.push((v >> 2) & 1),
                        "INIT_D" => value.push((v >> 3) & 1),
                        "INIT_E" => value.push((v >> 4) & 1),
                        "INIT_F" => value.push((v >> 5) & 1),
                        "INIT_G" => value.push((v >> 6) & 1),
                        "INIT_H" => value.push((v >> 7) & 1),
                        _ => (),
                    };
                }
                match p.name().as_str() {
                    "INIT_A" | "INIT_B" | "INIT_C" | "INIT_D" | "INIT_E" | "INIT_F" | "INIT_G"
                    | "INIT_H" => {
                        let param = ParamValue::Bytes(64, value);
                        map.insert(p.name(), param.to_expr());
                    }
                    _ => {
                        map.insert(p.name(), p.value().to_expr());
                    }
                };
            }
        } else {
            for p in self.to_param_set().iter() {
                map.insert(p.name(), p.value().to_expr());
            }
        }
        map
    }
    fn to_input_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        let id = self.instr().arg().get_id(0).unwrap();
        let ty = self.instr().arg().get_ty(0).unwrap();
        if let Some(width) = ty.width() {
            let mut concat = vl::ExprConcat::default();
            for _ in 0..width {
                concat.add_expr(vl::Expr::new_ref(GND));
            }
            for i in 0..(6_i32 - width as i32) {
                concat.add_expr(vl::Expr::new_index_bit(&id, i));
            }
            let expr = vl::Expr::from(concat);
            for p in self.prim.input().iter() {
                let name = p.name();
                match name.as_str() {
                    "ADDRA" | "ADDRB" | "ADDRC" | "ADDRD" | "ADDRE" | "ADDRF" | "ADDRG"
                    | "ADDRH" => map.insert(name, expr.clone()),
                    "WCLK" => map.insert(name, vl::Expr::new_ref(CLOCK)),
                    _ => map.insert(name, vl::Expr::new_ref(GND)),
                };
            }
        }
        map
    }
    fn to_output_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        let dst = self.instr().dst().get_id(0).unwrap();
        for p in self.prim.output().iter() {
            let name = p.name();
            match name.as_str() {
                "DOA" => map.insert(name, vl::Expr::new_index_bit(&dst, 0)),
                "DOB" => map.insert(name, vl::Expr::new_index_bit(&dst, 1)),
                "DOC" => map.insert(name, vl::Expr::new_index_bit(&dst, 2)),
                "DOD" => map.insert(name, vl::Expr::new_index_bit(&dst, 3)),
                "DOE" => map.insert(name, vl::Expr::new_index_bit(&dst, 4)),
                "DOF" => map.insert(name, vl::Expr::new_index_bit(&dst, 5)),
                "DOG" => map.insert(name, vl::Expr::new_index_bit(&dst, 6)),
                _ => map.insert(name, vl::Expr::new_index_bit(&dst, 7)),
            };
        }
        map
    }
    fn to_loc(&self) -> Option<&Loc> {
        self.instr.loc()
    }
}

// TODO: check for valid memory shapes, support only 8x8 now (data:i8, addr:i3)
pub fn rom_from_mach(instr: &InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let rom = Rom::new(instr.clone());
    Ok(rom.to_block())
}
