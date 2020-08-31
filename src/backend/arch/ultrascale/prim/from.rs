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
        match dsp.op() {
            DspOp::Add => {
                inst.connect("ALUMODE", verilog::Expr::new_ulit_bin(4, "0000"));
                inst.connect("INMODE", verilog::Expr::new_ulit_bin(5, "00000"));
                inst.connect("OPMODE", verilog::Expr::new_ulit_bin(9, "000110011"));
            }
            DspOp::Sub => {
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
        verilog::Stmt::from(inst)
    }
}
