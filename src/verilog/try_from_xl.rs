use crate::util::errors::Error;
use crate::verilog::ast as vl;
use crate::verilog::constant;
use crate::xl::ast as xl;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const LUT_INP: [&str; 6] = ["I0", "I1", "I2", "I3", "I4", "I5"];
const LUT_OUT: [&str; 1] = ["O"];
const CAR_INP: [&str; 4] = ["DI", "S", "CI", "CI_TOP"];
const CAR_OUT: [&str; 2] = ["O", "CO"];

fn gen_gnd_prim() -> vl::Stmt {
    let mut instance = vl::Instance::new("GND", "GND");
    instance.connect_ref("G", constant::GND);
    vl::Stmt::from(instance)
}

fn gen_vcc_prim() -> vl::Stmt {
    let mut instance = vl::Instance::new("VCC", "VCC");
    instance.connect_ref("P", constant::VCC);
    vl::Stmt::from(instance)
}

fn create_literal(value: i64, width: i64) -> vl::Expr {
    if width == 1 {
        let mask = value & 1;
        let is_one = mask == 1;
        if is_one {
            vl::Expr::new_ref(constant::VCC)
        } else {
            vl::Expr::new_ref(constant::GND)
        }
    } else {
        let mut concat = vl::ExprConcat::default();
        for i in 0..width {
            let shift = value >> i;
            let mask = shift & 1;
            let is_one = mask == 1;
            if is_one {
                concat.add_expr(vl::Expr::new_ref(constant::VCC));
            } else {
                concat.add_expr(vl::Expr::new_ref(constant::GND));
            }
        }
        vl::Expr::from(concat)
    }
}

fn instance_name_try_from_instr(instr: &xl::InstrMach) -> Result<vl::Id, Error> {
    let dst: Vec<vl::Id> = instr.dst().clone().try_into()?;
    Ok(format!("__{}", dst[0]))
}

fn lut_width_try_from_op(op: &xl::OpMach) -> Result<u32, Error> {
    match op {
        xl::OpMach::Lut1 => Ok(2),
        xl::OpMach::Lut2 => Ok(4),
        xl::OpMach::Lut3 => Ok(8),
        xl::OpMach::Lut4 => Ok(16),
        xl::OpMach::Lut5 => Ok(32),
        xl::OpMach::Lut6 => Ok(64),
        _ => Err(Error::new_conv_error("not a lut op")),
    }
}

fn lut_try_from_instr(instr: &xl::InstrMach) -> Result<vl::Stmt, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut instance = vl::Instance::new(&id, &prim);
    let dst: Vec<vl::Expr> = instr.dst().clone().try_into()?;
    let arg: Vec<vl::Expr> = instr.arg().clone().try_into()?;
    for (p, e) in LUT_INP.iter().zip(arg) {
        instance.connect(p, e.clone());
    }
    for (p, e) in LUT_OUT.iter().zip(dst) {
        instance.connect(p, e.clone());
    }
    if let Some(opt_val) = instr.opt_lookup(&xl::Opt::Table) {
        let table = format!("{:X}", u64::try_from(opt_val.clone())?);
        let width = lut_width_try_from_op(instr.op())?;
        instance.add_param("INIT", vl::Expr::new_ulit_hex(width, &table));
        if let Some(loc) = instr.loc() {
            let attr = vl::Attribute::from(loc.clone());
            instance.set_attr(attr);
        }
        Ok(vl::Stmt::from(instance))
    } else {
        Err(Error::new_conv_error("invalid lut2 option"))
    }
}

fn expr_try_from_term(term: &xl::ExprTerm, max_width: u64) -> Result<vl::Expr, Error> {
    let mut op = vl::ExprConcat::default();
    if let Some(id) = term.id() {
        if let Some(width) = term.width() {
            if let Some(length) = term.length() {
                if length > 0 && length <= 4 {
                    let rem = (max_width / length) - width;
                    for _ in 0..length {
                        op.add_expr(vl::Expr::new_ref(&id));
                        for _ in 0..rem {
                            op.add_expr(vl::Expr::new_ref(constant::GND));
                        }
                    }
                    Ok(vl::Expr::from(op))
                } else {
                    Err(Error::new_conv_error("vector length must be 1-4"))
                }
            } else {
                Err(Error::new_conv_error("not implemented yet"))
            }
        } else {
            Err(Error::new_conv_error("term must be var"))
        }
    } else {
        Err(Error::new_conv_error("term must be var"))
    }
}

fn dsp_try_from_instr(instr: &xl::InstrMach) -> Result<vl::Stmt, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut instance = vl::Instance::new(&id, &prim);
    if let Some(op) = instr.opt_op() {
        match op {
            xl::OpDsp::Add => {
                instance.add_param("USE_MULT", vl::Expr::new_str("NONE"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(51, 9));
                if let Some(e) = instr.arg().idx(0) {
                    instance.connect("C", expr_try_from_term(&e, 48)?);
                }
            }
            xl::OpDsp::Mul => {
                instance.add_param("USE_MULT", vl::Expr::new_str("MULTIPLY"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(5, 9));
            }
            xl::OpDsp::MulAdd => {
                instance.add_param("USE_MULT", vl::Expr::new_str("MULTIPLY"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(53, 9));
            }
        }
    }
    Ok(vl::Stmt::from(instance))
}

fn carry_try_from_instr(instr: &xl::InstrMach) -> Result<vl::Stmt, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut instance = vl::Instance::new(&id, &prim);
    let dst: Vec<vl::Expr> = instr.dst().clone().try_into()?;
    let arg: Vec<vl::Expr> = instr.arg().clone().try_into()?;
    for (p, e) in CAR_INP.iter().zip(arg) {
        instance.connect(p, e.clone());
    }
    for (p, e) in CAR_OUT.iter().zip(dst) {
        instance.connect(p, e.clone());
    }
    instance.add_param_str("CARRY_TYPE", "SINGLE_CY8");
    if let Some(loc) = instr.loc() {
        let attr = vl::Attribute::from(loc.clone());
        instance.set_attr(attr);
    }
    Ok(vl::Stmt::from(instance))
}

impl TryFrom<xl::OptVal> for u64 {
    type Error = Error;
    fn try_from(val: xl::OptVal) -> Result<Self, Self::Error> {
        match val {
            xl::OptVal::UInt(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}

impl TryFrom<xl::InstrBasc> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: xl::InstrBasc) -> Result<Self, Self::Error> {
        Ok(instr.dst().clone().try_into()?)
    }
}

impl TryFrom<xl::InstrMach> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: xl::InstrMach) -> Result<Self, Self::Error> {
        Ok(instr.dst().clone().try_into()?)
    }
}

impl TryFrom<xl::Instr> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: xl::Instr) -> Result<Self, Self::Error> {
        match instr {
            xl::Instr::Basc(instr) => Ok(instr.try_into()?),
            xl::Instr::Mach(instr) => Ok(instr.try_into()?),
        }
    }
}

impl TryFrom<xl::InstrBasc> for vl::Stmt {
    type Error = Error;
    fn try_from(instr: xl::InstrBasc) -> Result<Self, Self::Error> {
        if let Some(attr) = instr.attr().tup() {
            if let Some(expr) = instr.arg().tup() {
                let dst: Vec<vl::Expr> = instr.dst().clone().try_into()?;
                let arg: Vec<vl::Expr> = instr.arg().clone().try_into()?;
                let val: Vec<i64> = attr.clone().try_into()?;
                match instr.op() {
                    xl::OpBasc::Ext if expr.idx(0).is_vector() => {
                        if let Ok(idx) = usize::try_from(val[0]) {
                            Ok(vl::Stmt::from(vl::Parallel::Assign(
                                dst[0].clone(),
                                arg[idx].clone(),
                            )))
                        } else {
                            Err(Error::new_conv_error("invalid usize conversion"))
                        }
                    }
                    xl::OpBasc::Ext => {
                        if let Ok(idx) = i32::try_from(val[0]) {
                            Ok(vl::Stmt::from(vl::Parallel::Assign(
                                dst[0].clone(),
                                vl::Expr::new_index_bit(&arg[0].id(), idx),
                            )))
                        } else {
                            Err(Error::new_conv_error("invalid usize conversion"))
                        }
                    }
                    xl::OpBasc::Gnd => Ok(vl::Stmt::from(vl::Parallel::Assign(
                        dst[0].clone(),
                        vl::Expr::new_ref(constant::GND),
                    ))),
                    xl::OpBasc::Cat => {
                        let mut concat = vl::ExprConcat::default();
                        for a in arg {
                            concat.add_expr(a.clone());
                        }
                        Ok(vl::Stmt::from(vl::Parallel::Assign(
                            dst[0].clone(),
                            vl::Expr::from(concat),
                        )))
                    }
                    _ => Err(Error::new_conv_error("not implemented yet")),
                }
            } else {
                Err(Error::new_conv_error("not implemented yet"))
            }
        } else {
            Err(Error::new_conv_error("attr must be a tuple"))
        }
    }
}

impl TryFrom<xl::Instr> for vl::Stmt {
    type Error = Error;
    fn try_from(instr: xl::Instr) -> Result<Self, Self::Error> {
        match instr {
            xl::Instr::Basc(basc) => Ok(vl::Stmt::try_from(basc)?),
            xl::Instr::Mach(mach) => match mach.op() {
                xl::OpMach::Lut1
                | xl::OpMach::Lut2
                | xl::OpMach::Lut3
                | xl::OpMach::Lut4
                | xl::OpMach::Lut5
                | xl::OpMach::Lut6 => Ok(lut_try_from_instr(&mach)?),
                xl::OpMach::Carry => Ok(carry_try_from_instr(&mach)?),
                xl::OpMach::Dsp => Ok(dsp_try_from_instr(&mach)?),
                _ => Err(Error::new_conv_error("not implemented yet")),
            },
        }
    }
}

impl TryFrom<xl::Prog> for vl::Module {
    type Error = Error;
    fn try_from(prog: xl::Prog) -> Result<Self, Self::Error> {
        let mut module = vl::Module::try_from(prog.sig().clone())?;
        module.add_stmt(gen_gnd_prim());
        module.add_stmt(gen_vcc_prim());
        let mut decl: Vec<vl::Decl> = Vec::new();
        for i in prog.body() {
            let d: Vec<vl::Decl> = i.clone().try_into()?;
            decl.extend(d);
        }
        let decl_set: HashSet<vl::Decl> = decl.into_iter().collect();
        let output: Vec<vl::Decl> = prog.sig().output().clone().try_into()?;
        let output_set: HashSet<vl::Decl> = output.into_iter().collect();
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        for i in prog.body() {
            module.add_stmt(vl::Stmt::try_from(i.clone())?);
        }
        Ok(module)
    }
}
