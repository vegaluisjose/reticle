use crate::create_literal;
use crate::errors::Error;
use crate::inst_name_try_from_instr;
use crate::loc::Loc;
use crate::to_verilog::{ToVerilogExpr, ToVerilogInstance, VerilogExprMap};
use prim::ultrascale::bram::{Bram, ParamValue};
use prim::ultrascale::clock::CLOCK;
use prim::ultrascale::gnd::GND;
use prim::ultrascale::reset::RESET;
use prim::ultrascale::vcc::VCC;
use prim::{ParamSet, PortSet};
use verilog::ast as vl;
use xir::ast::InstrMach;

impl ToVerilogExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::CascadeOrder(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::ClockDomains(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::CollisionCheck(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::FilePath(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::RstRegPriority(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::WriteMode(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::Bool(v) => vl::Expr::new_ulit_bin(1, &format!("{}", *v as i32)),
            ParamValue::BoolNum(v) => vl::Expr::new_int(*v as i32),
            ParamValue::BoolStr(v) => vl::Expr::new_str(&format!("{}", v).to_uppercase()),
            ParamValue::Bytes(width, values) if values.is_empty() => {
                vl::Expr::new_ulit_hex(*width, "0")
            }
            ParamValue::Bytes(width, values) => {
                let mut num = String::new();
                for v in values.iter().rev() {
                    let val = format!("{:02X}", v);
                    num.push_str(&val);
                }
                vl::Expr::new_ulit_hex(*width, &num)
            }
            ParamValue::Num(v) => vl::Expr::new_int(*v as i32),
        }
    }
}

impl ToVerilogInstance<ParamValue> for Bram {
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
    pub prim: Bram,
    pub instr: InstrMach,
}

impl Rom {
    pub fn new(instr: InstrMach) -> Self {
        Rom {
            prim: Bram::default(),
            instr,
        }
    }
    pub fn instr(&self) -> &InstrMach {
        &self.instr
    }
}

fn init_mem(values: &[u8]) -> VerilogExprMap {
    let width = 32;
    let depth = 64;
    let bits = (width * 8) as u32;
    let mut map = VerilogExprMap::new();
    let mut values = values.to_vec();
    for i in 0..depth {
        let name = format!("INIT_{:02X}", i);
        let bytes = if values.len() >= width {
            let bytes: Vec<_> = values.drain(0..width).collect();
            bytes
        } else if !values.is_empty() {
            let pad = width - values.len();
            let mut bytes: Vec<_> = values.drain(..).collect();
            bytes.extend(vec![0; pad]);
            bytes
        } else {
            vec![0; width]
        };
        let param = ParamValue::Bytes(bits, bytes);
        map.insert(name, param.to_expr());
    }
    map
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
    fn to_loc(&self) -> Option<&Loc> {
        self.instr.loc()
    }
    fn to_param_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        if let Some(mem) = self.instr().mem() {
            let init_map = init_mem(mem.values());
            for p in self.to_param_set().iter() {
                if let Some(value) = init_map.get(&p.name()) {
                    map.insert(p.name(), value.clone());
                } else if p.name().as_str() == "READ_WIDTH_A" {
                    let param = ParamValue::Num(9);
                    map.insert(p.name(), param.to_expr());
                } else {
                    map.insert(p.name(), p.value().to_expr());
                }
            }
        } else {
            for p in self.to_param_set().iter() {
                if p.name().as_str() == "READ_WIDTH_A" {
                    let param = ParamValue::Num(9);
                    map.insert(p.name(), param.to_expr());
                } else {
                    map.insert(p.name(), p.value().to_expr());
                }
            }
        }
        map
    }
    fn to_input_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        let id = self.instr().arg().get_id(0).unwrap();
        let ty = self.instr().arg().get_ty(0).unwrap();
        if let Some(width) = ty.width() {
            let tail_pad = (width as f32).log(2.0) as u32;
            let mut concat = vl::ExprConcat::default();
            for _ in 0..tail_pad {
                concat.add_expr(vl::Expr::new_ref(GND));
            }
            concat.add_expr(vl::Expr::new_ref(&id));
            let head_pad = 14 - width as u32 - tail_pad;
            for _ in 0..head_pad {
                concat.add_expr(vl::Expr::new_ref(GND));
            }
            let expr = vl::Expr::from(concat);
            for p in self.prim.input().iter() {
                let name = p.name();
                match name.as_str() {
                    "ADDRARDADDR" => map.insert(name, expr.clone()),
                    "CLKARDCLK" => map.insert(name, vl::Expr::new_ref(CLOCK)),
                    "RSTRAMARSTRAM" => map.insert(name, vl::Expr::new_ref(RESET)),
                    "RSTREGARSTREG" => map.insert(name, vl::Expr::new_ref(RESET)),
                    "ENARDEN" => map.insert(name, vl::Expr::new_ref(VCC)),
                    _ => map.insert(name, create_literal(p.width() as u64, 0)),
                };
            }
        }
        map
    }
    fn to_output_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        let id = self.instr().dst().get_id(0).unwrap();
        for p in self.to_output_set().iter() {
            let name = p.name();
            match name.as_str() {
                "DOUTADOUT" => map.insert(name, vl::Expr::new_ref(&id)),
                _ => map.insert(name, vl::Expr::new_ref("")),
            };
        }
        map
    }
}

// TODO: check for valid memory shapes, support only 8x256 now (data:i8, addr:i8)
pub fn rom_from_mach(instr: &InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let rom = Rom::new(instr.clone());
    Ok(rom.to_block())
}
