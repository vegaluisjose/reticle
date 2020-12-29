use crate::ml::ast as ml;
use crate::util::errors::Error;
use crate::verilog::ast as verilog;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const LUT_INP: [&str; 6] = ["I0", "I1", "I2", "I3", "I4", "I5"];
const LUT_OUT: [&str; 1] = ["O"];

// fn expr_from_basc_op(instr: &ml::InstrBasc) -> Result<verilog::Expr, Error> {
//     match instr.op() {
//         ml::OpBasc::Ext => {
//             if let Some(attr) = instr.attr().tup() {
//                 if let Some(idx) = attr.idx(0).val() {
//                     if let Ok(udx) = usize::try_from(idx) {
//                         if let Some(arg) = instr.arg().tup() {
//                             let ids: Vec<verilog::Id> = arg.clone().into();
//                             if let Some(ty) = arg.idx(0).ty() {
//                                 if ty.is_vector() {
//                                     Ok(verilog::Expr::new_ref(&ids[udx]))
//                                 } else {
//                                     Ok(verilog::Expr::new_index_bit(&ids[0], udx as i32))
//                                 }
//                             } else {
//                                 Err(Error::new_conv_error("arg is not var expr"))
//                             }
//                         } else {
//                             Err(Error::new_conv_error("term arg not implemented"))
//                         }
//                     } else {
//                         Err(Error::new_conv_error("index overflow"))
//                     }
//                 } else {
//                     Err(Error::new_conv_error("attr is not a value"))
//                 }
//             } else {
//                 Err(Error::new_conv_error("attr must be a tuple"))
//             }
//         }
//         _ => Err(Error::new_conv_error("not implemented yet")),
//     }
// }

fn instance_name_try_from_instr(instr: &ml::InstrMach) -> Result<verilog::Id, Error> {
    let dst: Vec<verilog::Id> = instr.dst().clone().try_into()?;
    Ok(format!("__{}", dst[0]))
}

fn assign_try_from_instr(instr: &ml::InstrBasc) -> Result<(), Error> {
    if let Some(tup) = instr.attr().tup() {
        let vals: Vec<i64> = tup.clone().try_into()?;
        println!("{:?}", vals);
        Ok(())
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

fn lut_try_from_instr(instr: &ml::InstrMach) -> Result<verilog::Stmt, Error> {
    let prim: verilog::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut instance = verilog::Instance::new(&id, &prim);
    let dst: Vec<verilog::Expr> = instr.dst().clone().try_into()?;
    let arg: Vec<verilog::Expr> = instr.arg().clone().try_into()?;
    for (p, e) in LUT_INP.iter().zip(arg) {
        instance.connect(p, e.clone());
    }
    for (p, e) in LUT_OUT.iter().zip(dst) {
        instance.connect(p, e.clone());
    }
    if let Some(opt_val) = instr.opt_lookup(&ml::Opt::Table) {
        let table = format!("{:X}", u64::try_from(opt_val.clone())?);
        let width = lut_width_try_from_op(instr.op())?;
        instance.add_param("INIT", verilog::Expr::new_ulit_hex(width, &table));
        Ok(verilog::Stmt::from(instance))
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

impl TryFrom<ml::Instr> for verilog::Stmt {
    type Error = Error;
    fn try_from(instr: ml::Instr) -> Result<Self, Self::Error> {
        match instr {
            ml::Instr::Basc(basc) => {
                assign_try_from_instr(&basc)?;
                Ok(verilog::Stmt::from(verilog::Parallel::Assign(
                    verilog::Expr::new_ref("foo"),
                    verilog::Expr::new_ref("bar"),
                )))
            }
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

impl TryFrom<ml::Prog> for verilog::Module {
    type Error = Error;
    fn try_from(prog: ml::Prog) -> Result<Self, Self::Error> {
        let mut module = verilog::Module::from(prog.sig().clone());
        let output: Vec<verilog::Decl> = prog.sig().output().clone().into();
        let output_set: HashSet<verilog::Decl> = output.into_iter().collect();
        let mut decl: Vec<verilog::Decl> = Vec::new();
        for i in prog.body() {
            let d: Vec<verilog::Decl> = i.clone().into();
            decl.extend(d);
        }
        let decl_set: HashSet<verilog::Decl> = decl.into_iter().collect();
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        for i in prog.body() {
            module.add_stmt(verilog::Stmt::try_from(i.clone())?);
        }
        Ok(module)
    }
}
