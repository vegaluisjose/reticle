use crate::ml::ast as ml;
use crate::util::errors::Error;
use crate::verilog::ast as vl;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const GND: &str = "gnd";
const VCC: &str = "vcc";
const LUT_INP: [&str; 6] = ["I0", "I1", "I2", "I3", "I4", "I5"];
const LUT_OUT: [&str; 1] = ["O"];
const CAR_INP: [&str; 4] = ["DI", "S", "CI", "CI_TOP"];
const CAR_OUT: [&str; 2] = ["O", "CO"];

fn gen_gnd_prim() -> vl::Stmt {
    let mut instance = vl::Instance::new("GND", "GND");
    instance.connect_ref("G", GND);
    vl::Stmt::from(instance)
}

fn gen_vcc_prim() -> vl::Stmt {
    let mut instance = vl::Instance::new("VCC", "VCC");
    instance.connect_ref("P", VCC);
    vl::Stmt::from(instance)
}

fn instance_name_try_from_instr(instr: &ml::InstrMach) -> Result<vl::Id, Error> {
    let dst: Vec<vl::Id> = instr.dst().clone().try_into()?;
    Ok(format!("__{}", dst[0]))
}

fn lut_width_try_from_op(op: &ml::OpMach) -> Result<u32, Error> {
    match op {
        ml::OpMach::Lut1 => Ok(2),
        ml::OpMach::Lut2 => Ok(4),
        ml::OpMach::Lut3 => Ok(8),
        ml::OpMach::Lut4 => Ok(16),
        ml::OpMach::Lut5 => Ok(32),
        ml::OpMach::Lut6 => Ok(64),
        _ => Err(Error::new_conv_error("not a lut op")),
    }
}

fn lut_try_from_instr(instr: &ml::InstrMach) -> Result<vl::Stmt, Error> {
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
    if let Some(opt_val) = instr.opt_lookup(&ml::Opt::Table) {
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

fn dsp_try_from_instr(instr: &ml::InstrMach) -> Result<vl::Stmt, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let instance = vl::Instance::new(&id, &prim);
    Ok(vl::Stmt::from(instance))
}

fn carry_try_from_instr(instr: &ml::InstrMach) -> Result<vl::Stmt, Error> {
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

impl TryFrom<ml::OptVal> for u64 {
    type Error = Error;
    fn try_from(val: ml::OptVal) -> Result<Self, Self::Error> {
        match val {
            ml::OptVal::UInt(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}

impl TryFrom<ml::InstrBasc> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ml::InstrBasc) -> Result<Self, Self::Error> {
        Ok(instr.dst().clone().try_into()?)
    }
}

impl TryFrom<ml::InstrMach> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ml::InstrMach) -> Result<Self, Self::Error> {
        Ok(instr.dst().clone().try_into()?)
    }
}

impl TryFrom<ml::Instr> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ml::Instr) -> Result<Self, Self::Error> {
        match instr {
            ml::Instr::Basc(instr) => Ok(instr.try_into()?),
            ml::Instr::Mach(instr) => Ok(instr.try_into()?),
        }
    }
}

impl TryFrom<ml::InstrBasc> for vl::Stmt {
    type Error = Error;
    fn try_from(instr: ml::InstrBasc) -> Result<Self, Self::Error> {
        if let Some(attr) = instr.attr().tup() {
            if let Some(expr) = instr.arg().tup() {
                let dst: Vec<vl::Expr> = instr.dst().clone().try_into()?;
                let arg: Vec<vl::Expr> = instr.arg().clone().try_into()?;
                let val: Vec<i64> = attr.clone().try_into()?;
                match instr.op() {
                    ml::OpBasc::Ext if expr.idx(0).is_vector() => {
                        if let Ok(idx) = usize::try_from(val[0]) {
                            Ok(vl::Stmt::from(vl::Parallel::Assign(
                                dst[0].clone(),
                                arg[idx].clone(),
                            )))
                        } else {
                            Err(Error::new_conv_error("invalid usize conversion"))
                        }
                    }
                    ml::OpBasc::Ext => {
                        if let Ok(idx) = i32::try_from(val[0]) {
                            Ok(vl::Stmt::from(vl::Parallel::Assign(
                                dst[0].clone(),
                                vl::Expr::new_index_bit(&arg[0].id(), idx),
                            )))
                        } else {
                            Err(Error::new_conv_error("invalid usize conversion"))
                        }
                    }
                    ml::OpBasc::Gnd => Ok(vl::Stmt::from(vl::Parallel::Assign(
                        dst[0].clone(),
                        vl::Expr::new_ref(GND),
                    ))),
                    ml::OpBasc::Cat => {
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

impl TryFrom<ml::Instr> for vl::Stmt {
    type Error = Error;
    fn try_from(instr: ml::Instr) -> Result<Self, Self::Error> {
        match instr {
            ml::Instr::Basc(basc) => Ok(vl::Stmt::try_from(basc)?),
            ml::Instr::Mach(mach) => match mach.op() {
                ml::OpMach::Lut1
                | ml::OpMach::Lut2
                | ml::OpMach::Lut3
                | ml::OpMach::Lut4
                | ml::OpMach::Lut5
                | ml::OpMach::Lut6 => Ok(lut_try_from_instr(&mach)?),
                ml::OpMach::Carry => Ok(carry_try_from_instr(&mach)?),
                ml::OpMach::Dsp => Ok(dsp_try_from_instr(&mach)?),
                _ => Err(Error::new_conv_error("not implemented yet")),
            },
        }
    }
}

impl TryFrom<ml::Prog> for vl::Module {
    type Error = Error;
    fn try_from(prog: ml::Prog) -> Result<Self, Self::Error> {
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
