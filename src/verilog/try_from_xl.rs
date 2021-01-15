use crate::util::errors::Error;
use crate::verilog::ast as vl;
use crate::verilog::constant;
use crate::verilog::try_from_ir;
use crate::xl::ast as xl;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

const DSP_WIDTH_A: usize = 30;
const DSP_WIDTH_B: usize = 18;
const DSP_WIDTH_C: usize = 48;
const DSP_WIDTH_P: usize = 48;

const DSP_MAX_REG_A: i32 = 2;
const DSP_MAX_REG_B: i32 = 2;
const DSP_MAX_REG_C: i32 = 1;
const DSP_MAX_REG_M: i32 = 1;
const DSP_MAX_REG_P: i32 = 1;

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

fn reg_try_from_instr(instr: &xl::InstrMach) -> Result<vl::Stmt, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut instance = vl::Instance::new(&id, &prim);
    instance.connect("C", vl::Expr::new_ref(constant::CLOCK));
    match instr.op() {
        xl::OpMach::Fdre => instance.connect("R", vl::Expr::new_ref(constant::RESET)),
        xl::OpMach::Fdse => instance.connect("S", vl::Expr::new_ref(constant::RESET)),
        _ => (),
    }
    if let Some(e) = instr.arg().idx(0) {
        instance.connect("D", vl::Expr::new_ref(&String::try_from(e.clone())?));
    }
    if let Some(e) = instr.arg().idx(1) {
        instance.connect("CE", vl::Expr::new_ref(&String::try_from(e.clone())?));
    }
    if let Some(e) = instr.dst().idx(0) {
        instance.connect("Q", vl::Expr::new_ref(&String::try_from(e.clone())?));
    }
    if let Some(loc) = instr.loc() {
        let attr = vl::Attribute::from(loc.clone());
        instance.set_attr(attr);
    }
    Ok(vl::Stmt::from(instance))
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

fn vec_word_width_try_from_term(term: &xl::ExprTerm) -> Result<u64, Error> {
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

fn simd_opt_try_from_term(term: &xl::ExprTerm) -> Result<vl::Expr, Error> {
    if let Some(length) = term.length() {
        match length {
            4 => Ok(vl::Expr::new_str("FOUR12")),
            3 => Ok(vl::Expr::new_str("FOUR12")),
            2 => Ok(vl::Expr::new_str("TWO24")),
            1 => Ok(vl::Expr::new_str("ONE48")),
            _ => Err(Error::new_conv_error("unsupported length")),
        }
    } else {
        Ok(vl::Expr::new_str("ONE48"))
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

fn dsp_param_reg_try_from_instr(
    instr: &xl::InstrMach,
    reg: &xl::Opt,
    max: i32,
) -> Result<vl::Expr, Error> {
    if let Some(r) = instr.opt_lookup(reg) {
        let val = i32::try_from(r.clone())?;
        if val <= max {
            Ok(vl::Expr::new_int(val))
        } else {
            let err = format!("reg {} must be less or equal than {}", reg, max);
            Err(Error::new_conv_error(&err))
        }
    } else {
        Ok(vl::Expr::new_int(0))
    }
}

fn dsp_try_from_instr(instr: &xl::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let prim: vl::Id = instr.op().clone().into();
    let id = instance_name_try_from_instr(instr)?;
    let mut stmt: Vec<vl::Stmt> = Vec::new();
    let mut instance = vl::Instance::new(&id, &prim);
    if let Some(loc) = instr.loc() {
        let attr = vl::Attribute::from(loc.clone());
        instance.set_attr(attr);
    }
    // connect clock and reset
    instance.connect("CLK", vl::Expr::new_ref(constant::CLOCK));
    instance.connect("RSTA", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTALLCARRYIN", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTALUMODE", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTB", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTC", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTCTRL", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTD", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTINMODE", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTM", vl::Expr::new_ref(constant::RESET));
    instance.connect("RSTP", vl::Expr::new_ref(constant::RESET));
    // setup default params and ports
    instance.add_param("A_INPUT", vl::Expr::new_str("DIRECT"));
    instance.add_param("AMULTSEL", vl::Expr::new_str("A"));
    instance.add_param("B_INPUT", vl::Expr::new_str("DIRECT"));
    instance.add_param("BMULTSEL", vl::Expr::new_str("B"));
    instance.add_param("PREADDINSEL", vl::Expr::new_str("A"));
    instance.add_param("RND", vl::Expr::new_ulit_hex(48, "0"));
    instance.add_param("USE_WIDEXOR", vl::Expr::new_str("FALSE"));
    instance.add_param("XORSIMD", vl::Expr::new_str("XOR24_48_96"));
    instance.add_param("AUTORESET_PATDET", vl::Expr::new_str("NO_RESET"));
    instance.add_param("AUTORESET_PRIORITY", vl::Expr::new_str("RESET"));
    instance.add_param("MASK", vl::Expr::new_ulit_hex(48, "3fffffffffff"));
    instance.add_param("PATTERN", vl::Expr::new_ulit_hex(48, "0"));
    instance.add_param("SEL_MASK", vl::Expr::new_str("MASK"));
    instance.add_param("SEL_PATTERN", vl::Expr::new_str("PATTERN"));
    instance.add_param("USE_PATTERN_DETECT", vl::Expr::new_str("NO_PATDET"));
    instance.add_param("IS_ALUMODE_INVERTED", vl::Expr::new_ulit_bin(4, "0000"));
    instance.add_param("IS_CARRYIN_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_CLK_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_INMODE_INVERTED", vl::Expr::new_ulit_bin(5, "00000"));
    instance.add_param("IS_OPMODE_INVERTED", vl::Expr::new_ulit_bin(9, "000000000"));
    instance.add_param("IS_RSTALLCARRYIN_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTALUMODE_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTA_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTB_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTCTRL_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTC_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTD_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTINMODE_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTM_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.add_param("IS_RSTP_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
    instance.connect("CEAD", vl::Expr::new_ref(constant::GND));
    instance.connect("CEALUMODE", vl::Expr::new_ref(constant::GND));
    instance.connect("CECARRYIN", vl::Expr::new_ref(constant::GND));
    instance.connect("CECTRL", vl::Expr::new_ref(constant::GND));
    instance.connect("CED", vl::Expr::new_ref(constant::GND));
    instance.connect("CEINMODE", vl::Expr::new_ref(constant::GND));
    instance.add_param("ADREG", vl::Expr::new_int(0));
    instance.add_param("ALUMODEREG", vl::Expr::new_int(0));
    instance.add_param("CARRYINREG", vl::Expr::new_int(0));
    instance.add_param("CARRYINSELREG", vl::Expr::new_int(0));
    instance.add_param("DREG", vl::Expr::new_int(0));
    instance.add_param("INMODEREG", vl::Expr::new_int(0));
    instance.add_param("OPMODEREG", vl::Expr::new_int(0));
    instance.connect("ACIN", create_literal(0, 30));
    instance.connect("BCIN", create_literal(0, 18));
    instance.connect("CARRYCASCIN", vl::Expr::new_ref(constant::GND));
    instance.connect("MULTSIGNIN", vl::Expr::new_ref(constant::GND));
    instance.connect("PCIN", create_literal(0, 48));
    instance.connect("CARRYIN", vl::Expr::new_ref(constant::GND));
    instance.connect("CARRYINSEL", create_literal(0, 3));
    instance.connect("D", create_literal(0, 27));
    instance.connect("ACOUT", vl::Expr::new_ref(""));
    instance.connect("BCOUT", vl::Expr::new_ref(""));
    instance.connect("CARRYCASCOUT", vl::Expr::new_ref(""));
    instance.connect("MULTSIGNOUT", vl::Expr::new_ref(""));
    instance.connect("PCOUT", vl::Expr::new_ref(""));
    instance.connect("OVERFLOW", vl::Expr::new_ref(""));
    instance.connect("PATTERNBDETECT", vl::Expr::new_ref(""));
    instance.connect("PATTERNDETECT", vl::Expr::new_ref(""));
    instance.connect("UNDERFLOW", vl::Expr::new_ref(""));
    instance.connect("CARRYOUT", vl::Expr::new_ref(""));
    instance.connect("XOROUT", vl::Expr::new_ref(""));
    // setup SIMD option
    if let Some(e) = instr.dst().term() {
        instance.add_param("USE_SIMD", simd_opt_try_from_term(e)?);
    }
    // output seems op independent so far, this is likely to change
    if let Some(e) = instr.dst().idx(0) {
        let temp = tmp_name_try_from_term(e)?;
        instance.connect("P", vl::Expr::new_ref(&temp));
        let dst: Vec<vl::Expr> = e.clone().try_into()?;
        let word_width = vec_word_width_try_from_term(e)?;
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
    // set input ports and params based on operation
    if let Some(op) = instr.opt_op() {
        match op {
            xl::OpDsp::Add => {
                // RegA
                instance.add_param(
                    "CREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegA, DSP_MAX_REG_C)?,
                );
                // RegB
                instance.add_param(
                    "AREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_A)?,
                );
                instance.add_param(
                    "BREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_B)?,
                );
                instance.add_param(
                    "ACASCREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_A)?,
                );
                instance.add_param(
                    "BCASCREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_B)?,
                );
                // RegP
                instance.add_param(
                    "PREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegP, DSP_MAX_REG_P)?,
                );
                instance.add_param("MREG", vl::Expr::new_int(0));
                instance.add_param("USE_MULT", vl::Expr::new_str("NONE"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(51, 9));
                instance.connect("CEM", vl::Expr::new_ref(constant::GND));
                if let Some(e) = instr.arg().idx(0) {
                    let word_width = vec_word_width_try_from_term(e)?;
                    instance.connect("C", expr_try_from_term(&e, word_width, 0, DSP_WIDTH_C - 1)?);
                }
                if let Some(e) = instr.arg().idx(1) {
                    let word_width = vec_word_width_try_from_term(e)?;
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
                let i2 = instr.arg().idx(2);
                let i3 = instr.arg().idx(3);
                let i4 = instr.arg().idx(4);
                let ra = instr.opt_lookup(&xl::Opt::RegA);
                let rb = instr.opt_lookup(&xl::Opt::RegB);
                let rp = instr.opt_lookup(&xl::Opt::RegP);
                match (ra, rb, rp, i2, i3, i4) {
                    (None, None, None, None, None, None) => {
                        instance.connect("CEC", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEA1", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEA2", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEB1", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEB2", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEP", vl::Expr::new_ref(constant::GND));
                        stmt.push(vl::Stmt::from(instance));
                        Ok(stmt)
                    }
                    (Some(_), Some(_), Some(_), Some(e2), Some(e3), Some(e4)) => {
                        instance.connect("CEC", vl::Expr::new_ref(&String::try_from(e2.clone())?));
                        instance.connect("CEA1", vl::Expr::new_ref(&String::try_from(e3.clone())?));
                        instance.connect("CEA2", vl::Expr::new_ref(&String::try_from(e3.clone())?));
                        instance.connect("CEB1", vl::Expr::new_ref(&String::try_from(e3.clone())?));
                        instance.connect("CEB2", vl::Expr::new_ref(&String::try_from(e3.clone())?));
                        instance.connect("CEP", vl::Expr::new_ref(&String::try_from(e4.clone())?));
                        stmt.push(vl::Stmt::from(instance));
                        Ok(stmt)
                    }
                    (_, _, _, _, _, _) => {
                        Err(Error::new_conv_error("invalid dsp add configuration"))
                    }
                }
            }
            xl::OpDsp::MulAdd => {
                // RegA
                instance.add_param(
                    "AREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegA, DSP_MAX_REG_A)?,
                );
                instance.add_param(
                    "ACASCREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegA, DSP_MAX_REG_A)?,
                );
                // RegB
                instance.add_param(
                    "BREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_B)?,
                );
                instance.add_param(
                    "BCASCREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_B)?,
                );
                // RegC
                instance.add_param(
                    "CREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegC, DSP_MAX_REG_C)?,
                );
                // RegM
                instance.add_param(
                    "MREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegM, DSP_MAX_REG_M)?,
                );
                // RegP
                instance.add_param(
                    "PREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegP, DSP_MAX_REG_P)?,
                );
                instance.add_param("USE_MULT", vl::Expr::new_str("MULTIPLY"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(53, 9));
                if let Some(e) = instr.arg().idx(0) {
                    instance.connect(
                        "A",
                        expr_try_from_term(&e, DSP_WIDTH_A as u64, 0, DSP_WIDTH_A - 1)?,
                    );
                }
                if let Some(e) = instr.arg().idx(1) {
                    instance.connect(
                        "B",
                        expr_try_from_term(&e, DSP_WIDTH_B as u64, 0, DSP_WIDTH_B - 1)?,
                    );
                }
                if let Some(e) = instr.arg().idx(2) {
                    instance.connect(
                        "C",
                        expr_try_from_term(&e, DSP_WIDTH_C as u64, 0, DSP_WIDTH_C - 1)?,
                    );
                }
                let i3 = instr.arg().idx(3);
                let i4 = instr.arg().idx(4);
                let i5 = instr.arg().idx(5);
                let i6 = instr.arg().idx(6);
                let i7 = instr.arg().idx(7);
                let ra = instr.opt_lookup(&xl::Opt::RegA);
                let rb = instr.opt_lookup(&xl::Opt::RegB);
                let rc = instr.opt_lookup(&xl::Opt::RegC);
                let rm = instr.opt_lookup(&xl::Opt::RegM);
                let rp = instr.opt_lookup(&xl::Opt::RegP);
                match (ra, rb, rc, rm, rp, i3, i4, i5, i6, i7) {
                    (None, None, None, None, None, None, None, None, None, None) => {
                        instance.connect("CEA1", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEA2", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEB1", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEB2", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEC", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEM", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEP", vl::Expr::new_ref(constant::GND));
                        stmt.push(vl::Stmt::from(instance));
                        Ok(stmt)
                    }
                    (
                        Some(_),
                        Some(_),
                        None,
                        Some(_),
                        Some(_),
                        Some(e3),
                        Some(e4),
                        Some(e5),
                        Some(e6),
                        None,
                    ) => {
                        instance.connect("CEA1", vl::Expr::new_ref(&String::try_from(e3.clone())?));
                        instance.connect("CEA2", vl::Expr::new_ref(&String::try_from(e3.clone())?));
                        instance.connect("CEB1", vl::Expr::new_ref(&String::try_from(e4.clone())?));
                        instance.connect("CEB2", vl::Expr::new_ref(&String::try_from(e4.clone())?));
                        instance.connect("CEC", vl::Expr::new_ref(constant::GND));
                        instance.connect("CEM", vl::Expr::new_ref(&String::try_from(e5.clone())?));
                        instance.connect("CEP", vl::Expr::new_ref(&String::try_from(e6.clone())?));
                        stmt.push(vl::Stmt::from(instance));
                        Ok(stmt)
                    }
                    (_, _, _, _, _, _, _, _, _, _) => {
                        Err(Error::new_conv_error("invalid dsp muladd configuration"))
                    }
                }
            }
            xl::OpDsp::Mul => {
                // RegA
                instance.add_param(
                    "AREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegA, DSP_MAX_REG_A)?,
                );
                instance.add_param(
                    "ACASCREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegA, DSP_MAX_REG_A)?,
                );
                // RegB
                instance.add_param(
                    "BREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_B)?,
                );
                instance.add_param(
                    "BCASCREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegB, DSP_MAX_REG_B)?,
                );
                // RegM
                instance.add_param(
                    "MREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegM, DSP_MAX_REG_M)?,
                );
                // RegP
                instance.add_param(
                    "PREG",
                    dsp_param_reg_try_from_instr(instr, &xl::Opt::RegP, DSP_MAX_REG_P)?,
                );
                instance.add_param("CREG", vl::Expr::new_int(0));
                instance.add_param("USE_MULT", vl::Expr::new_str("MULTIPLY"));
                instance.connect("ALUMODE", create_literal(0, 4));
                instance.connect("INMODE", create_literal(0, 5));
                instance.connect("OPMODE", create_literal(5, 9));
                Err(Error::new_conv_error("dsp op not supported yet"))
            }
        }
    } else {
        Err(Error::new_conv_error("undefined op"))
    }
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

impl TryFrom<xl::InstrBasc> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: xl::InstrBasc) -> Result<Self, Self::Error> {
        Ok(try_from_ir::wire_try_from_expr(instr.dst().clone())?)
    }
}

impl TryFrom<xl::InstrMach> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: xl::InstrMach) -> Result<Self, Self::Error> {
        Ok(try_from_ir::wire_try_from_expr(instr.dst().clone())?)
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
                    xl::OpBasc::Ext => {
                        if let Some(v) = val.get(0) {
                            if let Ok(idx) = usize::try_from(*v) {
                                if let Some(a) = expr.idx(0) {
                                    if let Some(d) = dst.get(0) {
                                        if a.is_vector() {
                                            if let Some(i) = arg.get(idx) {
                                                Ok(vl::Stmt::from(vl::Parallel::Assign(
                                                    d.clone(),
                                                    i.clone(),
                                                )))
                                            } else {
                                                Err(Error::new_conv_error("elem out of bounds"))
                                            }
                                        } else {
                                            Ok(vl::Stmt::from(vl::Parallel::Assign(
                                                d.clone(),
                                                vl::Expr::new_index_bit(&arg[0].id(), idx as i32),
                                            )))
                                        }
                                    } else {
                                        Err(Error::new_conv_error("must have one dst"))
                                    }
                                } else {
                                    Err(Error::new_conv_error("must have one arg"))
                                }
                            } else {
                                Err(Error::new_conv_error("attr overflow"))
                            }
                        } else {
                            Err(Error::new_conv_error("must have one attr"))
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
                xl::OpMach::Fdre | xl::OpMach::Fdse => Ok(vec![reg_try_from_instr(&mach)?]),
            },
        }
    }
}

impl TryFrom<xl::Prog> for vl::Module {
    type Error = Error;
    fn try_from(prog: xl::Prog) -> Result<Self, Self::Error> {
        let mut module = vl::Module::try_from(prog.sig().clone())?;
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
        let output: Vec<vl::Decl> = try_from_ir::wire_try_from_expr(prog.sig().output().clone())?;
        let output_set: HashSet<vl::Decl> = output.into_iter().collect();
        module.add_decl(vl::Decl::new_wire(constant::GND, 1));
        module.add_decl(vl::Decl::new_wire(constant::VCC, 1));
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        module.add_stmt(gen_gnd_prim());
        module.add_stmt(gen_vcc_prim());
        for i in prog.body() {
            let stmt: Vec<vl::Stmt> = i.clone().try_into()?;
            for s in stmt {
                module.add_stmt(s.clone());
            }
        }
        Ok(module)
    }
}
