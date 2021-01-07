use crate::util::errors::Error;
use crate::verilog::ast as vl;
use crate::verilog::constant;
use crate::xl::ast as xl;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const DSP_WIDTH_A: usize = 30;
const DSP_WIDTH_B: usize = 18;
const DSP_WIDTH_C: usize = 48;
const DSP_WIDTH_P: usize = 48;

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

fn tmp_name_try_from_term(term: &xl::ExprTerm) -> Result<xl::Id, Error> {
    let dst: xl::Id = term.clone().try_into()?;
    Ok(format!("_{}", dst))
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

fn expr_try_from_term(
    term: &xl::ExprTerm,
    word_width: u64,
    lsb: usize,
    msb: usize,
) -> Result<vl::Expr, Error> {
    let mut bits: Vec<vl::Expr> = Vec::new();
    let expr: Vec<vl::Expr> = term.clone().try_into()?;
    if let Some(width) = term.width() {
        for e in expr {
            if let Ok(wbits) = i32::try_from(width) {
                if let Ok(pbits) = i32::try_from(word_width - width) {
                    for i in 0..wbits {
                        bits.push(vl::Expr::new_index_bit(&e.id(), i));
                    }
                    for _ in 0..pbits {
                        bits.push(vl::Expr::new_ref(constant::GND));
                    }
                }
            }
        }
        let mut cat = vl::ExprConcat::default();
        for i in bits.iter().take(msb + 1).skip(lsb) {
            cat.add_expr(i.clone());
        }
        Ok(vl::Expr::from(cat))
    } else {
        Err(Error::new_conv_error("term must be var"))
    }
}

fn word_width_try_from_term(term: &xl::ExprTerm) -> Result<u64, Error> {
    if let Some(length) = term.length() {
        match length {
            4 => Ok(12),
            3 => Ok(12),
            2 => Ok(24),
            1 => Ok(48),
            _ => Err(Error::new_conv_error("unsupported length")),
        }
    } else {
        Ok(48)
    }
}

fn dsp_tmp_decl_try_from_instr(instr: &xl::InstrMach) -> Result<vl::Decl, Error> {
    match instr.op() {
        xl::OpMach::Dsp => {
            if let Some(e) = instr.dst().term() {
                let name = tmp_name_try_from_term(e)?;
                if let Ok(width) = u64::try_from(DSP_WIDTH_P) {
                    Ok(vl::Decl::new_wire(&name, width))
                } else {
                    Err(Error::new_conv_error("converting width constant"))
                }
            } else {
                Err(Error::new_conv_error("dsp instr supports only output term"))
            }
        }
        _ => Err(Error::new_conv_error("not a dsp instr")),
    }
}

fn dsp_try_from_instr(instr: &xl::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut instance = vl::Instance::new(&id, &prim);
    let mut stmt: Vec<vl::Stmt> = Vec::new();
    if let Some(op) = instr.opt_op() {
        match op {
            xl::OpDsp::Add => {
                instance.add_param("USE_MULT", vl::Expr::new_str("NONE"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(51, 9));
                if let Some(e) = instr.arg().idx(0) {
                    let word_width = word_width_try_from_term(e)?;
                    instance.connect("C", expr_try_from_term(&e, word_width, 0, DSP_WIDTH_C - 1)?);
                }
                if let Some(e) = instr.arg().idx(1) {
                    let word_width = word_width_try_from_term(e)?;
                    instance.connect("B", expr_try_from_term(&e, word_width, 0, DSP_WIDTH_B - 1)?);
                    instance.connect(
                        "A",
                        expr_try_from_term(
                            &e,
                            word_width,
                            DSP_WIDTH_B,
                            DSP_WIDTH_B + DSP_WIDTH_A - 1,
                        )?,
                    );
                }
                if let Some(e) = instr.dst().term() {
                    let temp = tmp_name_try_from_term(e)?;
                    instance.connect("P", vl::Expr::new_ref(&temp));
                    let dst: Vec<vl::Expr> = e.clone().try_into()?;
                    let word_width = word_width_try_from_term(e)?;
                    if let Some(width) = e.width() {
                        if let Ok(ebits) = i32::try_from(width) {
                            if let Ok(wbits) = i32::try_from(word_width) {
                                for (i, e) in dst.iter().enumerate() {
                                    let i = i as i32;
                                    let hi = i * wbits + (ebits - 1);
                                    let lo = i * wbits;
                                    let assign = vl::Parallel::Assign(
                                        e.clone(),
                                        vl::Expr::new_slice(
                                            &temp,
                                            vl::Expr::new_int(hi),
                                            vl::Expr::new_int(lo),
                                        ),
                                    );
                                    stmt.push(vl::Stmt::from(assign));
                                }
                            }
                        }
                    }
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
    stmt.push(vl::Stmt::from(instance));
    Ok(stmt)
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

impl TryFrom<xl::Instr> for Vec<vl::Stmt> {
    type Error = Error;
    fn try_from(instr: xl::Instr) -> Result<Self, Self::Error> {
        match instr {
            xl::Instr::Basc(basc) => Ok(vec![vl::Stmt::try_from(basc)?]),
            xl::Instr::Mach(mach) => match mach.op() {
                xl::OpMach::Lut1
                | xl::OpMach::Lut2
                | xl::OpMach::Lut3
                | xl::OpMach::Lut4
                | xl::OpMach::Lut5
                | xl::OpMach::Lut6 => Ok(vec![lut_try_from_instr(&mach)?]),
                xl::OpMach::Carry => Ok(vec![carry_try_from_instr(&mach)?]),
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
            if let Some(instr) = i.mach() {
                if let Ok(d) = dsp_tmp_decl_try_from_instr(instr) {
                    decl.push(d);
                }
            }
        }
        let decl_set: HashSet<vl::Decl> = decl.into_iter().collect();
        let output: Vec<vl::Decl> = prog.sig().output().clone().try_into()?;
        let output_set: HashSet<vl::Decl> = output.into_iter().collect();
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        for i in prog.body() {
            let stmt: Vec<vl::Stmt> = i.clone().try_into()?;
            for s in stmt {
                module.add_stmt(s.clone());
            }
        }
        Ok(module)
    }
}
