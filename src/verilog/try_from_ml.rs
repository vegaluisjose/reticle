use crate::ml::ast as ml;
use crate::util::errors::Error;
use crate::verilog::ast as vl;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const LUT_INP: [&str; 6] = ["I0", "I1", "I2", "I3", "I4", "I5"];
const LUT_OUT: [&str; 1] = ["O"];

fn instance_name_try_from_instr(instr: &ml::InstrMach) -> Result<vl::Id, Error> {
    let dst: Vec<vl::Id> = instr.dst().clone().try_into()?;
    Ok(format!("__{}", dst[0]))
}

fn assign_try_from_instr(instr: &ml::InstrBasc) -> Result<vl::Stmt, Error> {
    if let Some(attr) = instr.attr().tup() {
        if let Some(expr) = instr.arg().tup() {
            let dst: Vec<vl::Expr> = instr.dst().clone().try_into()?;
            let arg: Vec<vl::Expr> = instr.arg().clone().try_into()?;
            let val: Vec<i64> = attr.clone().try_into()?;
            if let Ok(idx) = usize::try_from(val[0]) {
                if expr.idx(0).is_vector() {
                    Ok(vl::Stmt::from(vl::Parallel::Assign(
                        dst[0].clone(),
                        arg[idx].clone(),
                    )))
                } else {
                    Ok(vl::Stmt::from(vl::Parallel::Assign(
                        dst[0].clone(),
                        vl::Expr::new_index_bit(&arg[0].id(), idx as i32),
                    )))
                }
            } else {
                Err(Error::new_conv_error("invalid usize conversion"))
            }
        } else {
            Err(Error::new_conv_error("not implemented yet"))
        }
    } else {
        Err(Error::new_conv_error("attr must be a tuple"))
    }
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
        Ok(vl::Stmt::from(instance))
    } else {
        Err(Error::new_conv_error("invalid lut2 option"))
    }
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

impl TryFrom<ml::Instr> for vl::Stmt {
    type Error = Error;
    fn try_from(instr: ml::Instr) -> Result<Self, Self::Error> {
        match instr {
            ml::Instr::Basc(basc) => Ok(assign_try_from_instr(&basc)?),
            ml::Instr::Mach(mach) => match mach.op() {
                ml::OpMach::Lut1
                | ml::OpMach::Lut2
                | ml::OpMach::Lut3
                | ml::OpMach::Lut4
                | ml::OpMach::Lut5
                | ml::OpMach::Lut6 => Ok(lut_try_from_instr(&mach)?),
                _ => Err(Error::new_conv_error("not implemented yet")),
            },
        }
    }
}

impl TryFrom<ml::Prog> for vl::Module {
    type Error = Error;
    fn try_from(prog: ml::Prog) -> Result<Self, Self::Error> {
        let mut decl: Vec<vl::Decl> = Vec::new();
        for i in prog.body() {
            let d: Vec<vl::Decl> = i.clone().try_into()?;
            decl.extend(d);
        }
        let decl_set: HashSet<vl::Decl> = decl.into_iter().collect();
        let output: Vec<vl::Decl> = prog.sig().output().clone().try_into()?;
        let output_set: HashSet<vl::Decl> = output.into_iter().collect();
        let mut module = vl::Module::try_from(prog.sig().clone())?;
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        for i in prog.body() {
            module.add_stmt(vl::Stmt::try_from(i.clone())?);
        }
        Ok(module)
    }
}
