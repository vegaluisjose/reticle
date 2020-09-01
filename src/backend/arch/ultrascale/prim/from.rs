use crate::backend::arch::ultrascale::prim::ast::*;
use crate::backend::verilog;

fn lut_width(ty: LutTy) -> u32 {
    match ty {
        LutTy::Lut2 => 4,
        LutTy::Lut3 => 8,
        LutTy::Lut4 => 16,
        LutTy::Lut5 => 32,
        LutTy::Lut6 => 64,
    }
}

impl From<Expr> for verilog::Expr {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Ref(name) => verilog::Expr::new_ref(&name),
            Expr::Index(name, index) => verilog::Expr::new_bit(&name, index as i32),
        }
    }
}

impl From<Lut> for verilog::Stmt {
    fn from(lut: Lut) -> Self {
        let mut inst = verilog::Instance::new(&lut.id(), &lut.ty().to_string());
        let width = lut_width(lut.ty().clone());
        inst.add_param("INIT", verilog::Expr::new_ulit_hex(width, &lut.init()));
        for (i, input) in lut.inputs().iter().enumerate() {
            let port = format!("I{}", i);
            inst.connect(&port, verilog::Expr::from(input.clone()));
        }
        inst.connect("O", verilog::Expr::from(lut.output().clone()));
        verilog::Stmt::from(inst)
    }
}

impl From<Reg> for verilog::Stmt {
    fn from(reg: Reg) -> Self {
        let mut inst = verilog::Instance::new(&reg.id(), &reg.ty().to_string());
        inst.add_param("IS_C_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_D_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        if reg.is_fdre() {
            inst.add_param("INIT", verilog::Expr::new_ulit_bin(1, "0"));
            inst.add_param("IS_R_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        } else {
            inst.add_param("INIT", verilog::Expr::new_ulit_bin(1, "1"));
            inst.add_param("IS_S_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        }
        inst.connect("C", verilog::Expr::from(reg.clock().clone()));
        inst.connect("CE", verilog::Expr::from(reg.en().clone()));
        inst.connect("D", verilog::Expr::from(reg.input().clone()));
        inst.connect("Q", verilog::Expr::from(reg.output().clone()));
        if reg.is_fdre() {
            inst.connect("R", verilog::Expr::from(reg.reset().clone()));
        } else {
            inst.connect("S", verilog::Expr::from(reg.reset().clone()));
        }
        verilog::Stmt::from(inst)
    }
}

impl From<Dsp> for verilog::Stmt {
    fn from(dsp: Dsp) -> Self {
        let mut inst = verilog::Instance::new(&dsp.id(), "DSP48E2");
        inst.connect("CLK", verilog::Expr::from(dsp.clock().clone()));
        inst.connect("RSTA", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTALLCARRYIN", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTALUMODE", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTB", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTC", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTCTRL", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTD", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTINMODE", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTM", verilog::Expr::from(dsp.reset().clone()));
        inst.connect("RSTP", verilog::Expr::from(dsp.reset().clone()));
        if dsp.en().is_default() {
            inst.connect("CEA1", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEA2", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEAD", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEALUMODE", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEB1", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEB2", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEC", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CECARRYIN", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CECTRL", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CED", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEINMODE", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEM", verilog::Expr::new_ulit_bin(1, "0"));
            inst.connect("CEP", verilog::Expr::new_ulit_bin(1, "0"));
        } else {
            inst.connect("CEA1", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEA2", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEAD", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEALUMODE", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEB1", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEB2", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEC", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CECARRYIN", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CECTRL", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CED", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEINMODE", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEM", verilog::Expr::from(dsp.en().clone()));
            inst.connect("CEP", verilog::Expr::from(dsp.en().clone()));
        }
        match dsp.op() {
            DspOp::Add => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", verilog::Expr::new_ulit_bin(4, "0000"));
                inst.connect("INMODE", verilog::Expr::new_ulit_bin(5, "00000"));
                inst.connect("OPMODE", verilog::Expr::new_ulit_bin(9, "000110011"));
            }
            DspOp::Sub => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", verilog::Expr::new_ulit_bin(4, "0011"));
                inst.connect("INMODE", verilog::Expr::new_ulit_bin(5, "00000"));
                inst.connect("OPMODE", verilog::Expr::new_ulit_bin(9, "000110011"));
            }
            _ => (),
        }
        match dsp.ty() {
            DspTy::Scalar => inst.add_param("USE_SIMD", verilog::Expr::new_str("ONE48")),
            DspTy::Vector(2) => inst.add_param("USE_SIMD", verilog::Expr::new_str("TWO24")),
            DspTy::Vector(3) => inst.add_param("USE_SIMD", verilog::Expr::new_str("FOUR12")),
            DspTy::Vector(4) => inst.add_param("USE_SIMD", verilog::Expr::new_str("FOUR12")),
            DspTy::Vector(_) => unimplemented!(),
        }
        // default params
        inst.add_param("A_INPUT", verilog::Expr::new_str("DIRECT"));
        inst.add_param("AMULTSEL", verilog::Expr::new_str("A"));
        inst.add_param("B_INPUT", verilog::Expr::new_str("DIRECT"));
        inst.add_param("BMULTSEL", verilog::Expr::new_str("B"));
        inst.add_param("PREADDINSEL", verilog::Expr::new_str("A"));
        inst.add_param("RND", verilog::Expr::new_ulit_hex(48, "0"));
        inst.add_param("USE_WIDEXOR", verilog::Expr::new_str("FALSE"));
        inst.add_param("XORSIMD", verilog::Expr::new_str("XOR24_48_96"));
        inst.add_param("AUTORESET_PATDET", verilog::Expr::new_str("NO_RESET"));
        inst.add_param("AUTORESET_PRIORITY", verilog::Expr::new_str("RESET"));
        inst.add_param("MASK", verilog::Expr::new_ulit_hex(48, "3fffffffffff"));
        inst.add_param("PATTERN", verilog::Expr::new_ulit_hex(48, "0"));
        inst.add_param("SEL_MASK", verilog::Expr::new_str("MASK"));
        inst.add_param("SEL_PATTERN", verilog::Expr::new_str("PATTERN"));
        inst.add_param("USE_PATTERN_DETECT", verilog::Expr::new_str("NO_PATDET"));
        inst.add_param(
            "IS_ALUMODE_INVERTED",
            verilog::Expr::new_ulit_bin(4, "0000"),
        );
        inst.add_param("IS_CARRYIN_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_CLK_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param(
            "IS_INMODE_INVERTED",
            verilog::Expr::new_ulit_bin(5, "00000"),
        );
        inst.add_param(
            "IS_OPMODE_INVERTED",
            verilog::Expr::new_ulit_bin(9, "000000000"),
        );
        inst.add_param(
            "IS_RSTALLCARRYIN_INVERTED",
            verilog::Expr::new_ulit_bin(1, "0"),
        );
        inst.add_param(
            "IS_RSTALUMODE_INVERTED",
            verilog::Expr::new_ulit_bin(1, "0"),
        );
        inst.add_param("IS_RSTA_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTB_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTCTRL_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTC_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTD_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTINMODE_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTM_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTP_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        // default registers (likely to be changed, based on options)
        inst.add_param("ACASCREG", verilog::Expr::new_int(0));
        inst.add_param("ADREG", verilog::Expr::new_int(0));
        inst.add_param("ALUMODEREG", verilog::Expr::new_int(0));
        inst.add_param("AREG", verilog::Expr::new_int(0));
        inst.add_param("BCASCREG", verilog::Expr::new_int(0));
        inst.add_param("BREG", verilog::Expr::new_int(0));
        inst.add_param("CARRYINREG", verilog::Expr::new_int(0));
        inst.add_param("CARRYINSELREG", verilog::Expr::new_int(0));
        inst.add_param("CREG", verilog::Expr::new_int(0));
        inst.add_param("DREG", verilog::Expr::new_int(0));
        inst.add_param("INMODEREG", verilog::Expr::new_int(0));
        inst.add_param("MREG", verilog::Expr::new_int(0));
        inst.add_param("OPMODEREG", verilog::Expr::new_int(0));
        inst.add_param("PREG", verilog::Expr::new_int(0));
        verilog::Stmt::from(inst)
    }
}
