use crate::errors::Error;
use crate::expr::ToExpr;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelDsp, ExprCoord, Loc};
use crate::port::{Input, Output};
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub enum InputTy {
    Direct,
    Cascade,
}

#[derive(Clone, Debug)]
pub enum AMultSel {
    A,
    AD,
}

#[derive(Clone, Debug)]
pub enum BMultSel {
    B,
    AD,
}

#[derive(Clone, Debug)]
pub enum PreAddInSel {
    A,
    B,
}

#[derive(Clone, Debug)]
pub enum UseMult {
    Multiply,
    Dynamic,
    None,
}

#[derive(Clone, Debug)]
pub enum UseSimd {
    One,
    Two,
    Four,
}

#[derive(Clone, Debug)]
pub enum XorSimd {
    One,
    Two,
}

#[derive(Clone, Debug)]
pub enum AutoResetPatDet {
    NoReset,
    ResetMatch,
    ResetNotMatch,
}

#[derive(Clone, Debug)]
pub enum AutoResetPriority {
    Reset,
    Cep,
}

#[derive(Clone, Debug)]
pub enum SelMask {
    C,
    Mask,
    RoundModeOne,
    RoundModeTwo,
}

#[derive(Clone, Debug)]
pub enum SelPattern {
    C,
    Pattern,
}

#[derive(Clone, Debug)]
pub enum UsePatternDetect {
    NoPatDet,
    PatDet,
}

#[derive(Clone, Debug)]
pub enum NumReg {
    Zero,
    One,
}

#[derive(Clone, Debug)]
pub enum NumRegAB {
    Zero,
    One,
    Two,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub a_input: InputTy,
    pub b_input: InputTy,
    pub a_multsel: AMultSel,
    pub b_multsel: BMultSel,
    pub preaddinsel: PreAddInSel,
    pub rnd: u64,
    pub use_mult: UseMult,
    pub use_simd: UseSimd,
    pub use_widexor: bool,
    pub xorsimd: XorSimd,
    pub autoreset_patdet: AutoResetPatDet,
    pub autoreset_priority: AutoResetPriority,
    pub mask: u64,
    pub pattern: u64,
    pub sel_mask: SelMask,
    pub sel_pattern: SelPattern,
    pub use_pattern_detect: UsePatternDetect,
    pub is_alumode_inverted: u64,
    pub is_carryin_inverted: bool,
    pub is_clk_inverted: bool,
    pub is_inmode_inverted: u64,
    pub is_opmode_inverted: u64,
    pub is_rstallcarryin_inverted: bool,
    pub is_rstalumode_inverted: bool,
    pub is_rsta_inverted: bool,
    pub is_rstb_inverted: bool,
    pub is_rstctrl_inverted: bool,
    pub is_rstc_inverted: bool,
    pub is_rstd_inverted: bool,
    pub is_rstinmode_inverted: bool,
    pub is_rstm_inverted: bool,
    pub is_rstp_inverted: bool,
    pub acascreg: NumRegAB,
    pub adreg: NumReg,
    pub alumodereg: NumReg,
    pub areg: NumRegAB,
    pub bcascreg: NumRegAB,
    pub breg: NumRegAB,
    pub carryinreg: NumReg,
    pub carryinselreg: NumReg,
    pub creg: NumReg,
    pub dreg: NumReg,
    pub inmodereg: NumReg,
    pub mreg: NumReg,
    pub opmodereg: NumReg,
    pub preg: NumReg,
}

#[derive(Clone, Debug)]
pub struct Dsp {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

impl Dsp {
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
}

impl Default for InputTy {
    fn default() -> Self {
        InputTy::Direct
    }
}

impl Default for AMultSel {
    fn default() -> Self {
        AMultSel::A
    }
}

impl Default for BMultSel {
    fn default() -> Self {
        BMultSel::B
    }
}

impl Default for PreAddInSel {
    fn default() -> Self {
        PreAddInSel::A
    }
}

impl Default for UseMult {
    fn default() -> Self {
        UseMult::Multiply
    }
}

impl Default for UseSimd {
    fn default() -> Self {
        UseSimd::One
    }
}

impl Default for XorSimd {
    fn default() -> Self {
        XorSimd::Two
    }
}

impl Default for AutoResetPatDet {
    fn default() -> Self {
        AutoResetPatDet::NoReset
    }
}

impl Default for AutoResetPriority {
    fn default() -> Self {
        AutoResetPriority::Reset
    }
}

impl Default for SelMask {
    fn default() -> Self {
        SelMask::Mask
    }
}

impl Default for SelPattern {
    fn default() -> Self {
        SelPattern::Pattern
    }
}

impl Default for UsePatternDetect {
    fn default() -> Self {
        UsePatternDetect::NoPatDet
    }
}

impl Default for NumReg {
    fn default() -> Self {
        NumReg::Zero
    }
}

impl Default for NumRegAB {
    fn default() -> Self {
        NumRegAB::Zero
    }
}

impl Default for Attr {
    fn default() -> Self {
        Attr {
            a_input: InputTy::default(),
            b_input: InputTy::default(),
            a_multsel: AMultSel::default(),
            b_multsel: BMultSel::default(),
            preaddinsel: PreAddInSel::default(),
            rnd: 0,
            use_mult: UseMult::default(),
            use_simd: UseSimd::default(),
            use_widexor: false,
            xorsimd: XorSimd::default(),
            autoreset_patdet: AutoResetPatDet::default(),
            autoreset_priority: AutoResetPriority::default(),
            mask: u64::from_str_radix("3fffffffffff", 16).unwrap(),
            pattern: 0,
            sel_mask: SelMask::default(),
            sel_pattern: SelPattern::default(),
            use_pattern_detect: UsePatternDetect::default(),
            is_alumode_inverted: 0,
            is_carryin_inverted: false,
            is_clk_inverted: false,
            is_inmode_inverted: 0,
            is_opmode_inverted: 0,
            is_rstallcarryin_inverted: false,
            is_rstalumode_inverted: false,
            is_rsta_inverted: false,
            is_rstb_inverted: false,
            is_rstctrl_inverted: false,
            is_rstc_inverted: false,
            is_rstd_inverted: false,
            is_rstinmode_inverted: false,
            is_rstm_inverted: false,
            is_rstp_inverted: false,
            acascreg: NumRegAB::Zero,
            adreg: NumReg::Zero,
            alumodereg: NumReg::Zero,
            areg: NumRegAB::Zero,
            bcascreg: NumRegAB::Zero,
            breg: NumRegAB::Zero,
            carryinreg: NumReg::Zero,
            carryinselreg: NumReg::Zero,
            creg: NumReg::Zero,
            dreg: NumReg::Zero,
            inmodereg: NumReg::Zero,
            mreg: NumReg::Zero,
            opmodereg: NumReg::Zero,
            preg: NumReg::Zero,
        }
    }
}

impl Default for Dsp {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Dsp(BelDsp::Alu),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Dsp {
            name: String::new(),
            prim: "DSP48E2".to_string(),
            loc,
            attr: Attr::default(),
            input: Input::dsp(),
            output: Output::dsp(),
        }
    }
}

impl ToExpr for InputTy {
    fn to_expr(&self) -> vl::Expr {
        match self {
            InputTy::Direct => vl::Expr::new_str("DIRECT"),
            InputTy::Cascade => vl::Expr::new_str("CASCADE"),
        }
    }
}

impl ToExpr for AMultSel {
    fn to_expr(&self) -> vl::Expr {
        match self {
            AMultSel::A => vl::Expr::new_str("A"),
            AMultSel::AD => vl::Expr::new_str("AD"),
        }
    }
}

impl ToExpr for BMultSel {
    fn to_expr(&self) -> vl::Expr {
        match self {
            BMultSel::B => vl::Expr::new_str("B"),
            BMultSel::AD => vl::Expr::new_str("AD"),
        }
    }
}

impl ToExpr for PreAddInSel {
    fn to_expr(&self) -> vl::Expr {
        match self {
            PreAddInSel::A => vl::Expr::new_str("A"),
            PreAddInSel::B => vl::Expr::new_str("B"),
        }
    }
}

impl ToExpr for UseMult {
    fn to_expr(&self) -> vl::Expr {
        match self {
            UseMult::Multiply => vl::Expr::new_str("MULTIPLY"),
            UseMult::Dynamic => vl::Expr::new_str("DYNAMIC"),
            UseMult::None => vl::Expr::new_str("NONE"),
        }
    }
}

impl ToExpr for UseSimd {
    fn to_expr(&self) -> vl::Expr {
        match self {
            UseSimd::One => vl::Expr::new_str("ONE48"),
            UseSimd::Two => vl::Expr::new_str("TWO24"),
            UseSimd::Four => vl::Expr::new_str("FOUR12"),
        }
    }
}

impl ToExpr for XorSimd {
    fn to_expr(&self) -> vl::Expr {
        match self {
            XorSimd::One => vl::Expr::new_str("XOR12"),
            XorSimd::Two => vl::Expr::new_str("XOR24_48_96"),
        }
    }
}

impl ToExpr for AutoResetPatDet {
    fn to_expr(&self) -> vl::Expr {
        match self {
            AutoResetPatDet::NoReset => vl::Expr::new_str("NO_RESET"),
            AutoResetPatDet::ResetMatch => vl::Expr::new_str("RESET_MATCH"),
            AutoResetPatDet::ResetNotMatch => vl::Expr::new_str("RESET_NOT_MATCH"),
        }
    }
}

impl ToExpr for AutoResetPriority {
    fn to_expr(&self) -> vl::Expr {
        match self {
            AutoResetPriority::Reset => vl::Expr::new_str("RESET"),
            AutoResetPriority::Cep => vl::Expr::new_str("CEP"),
        }
    }
}

impl ToExpr for SelMask {
    fn to_expr(&self) -> vl::Expr {
        match self {
            SelMask::C => vl::Expr::new_str("C"),
            SelMask::Mask => vl::Expr::new_str("MASK"),
            SelMask::RoundModeOne => vl::Expr::new_str("ROUNDING_MODE1"),
            SelMask::RoundModeTwo => vl::Expr::new_str("ROUNDING_MODE2"),
        }
    }
}

impl ToExpr for SelPattern {
    fn to_expr(&self) -> vl::Expr {
        match self {
            SelPattern::C => vl::Expr::new_str("C"),
            SelPattern::Pattern => vl::Expr::new_str("PATTERN"),
        }
    }
}

impl ToExpr for UsePatternDetect {
    fn to_expr(&self) -> vl::Expr {
        match self {
            UsePatternDetect::NoPatDet => vl::Expr::new_str("NO_PATDET"),
            UsePatternDetect::PatDet => vl::Expr::new_str("PATDET"),
        }
    }
}

impl ToExpr for NumReg {
    fn to_expr(&self) -> vl::Expr {
        match self {
            NumReg::Zero => vl::Expr::new_int(0),
            NumReg::One => vl::Expr::new_int(1),
        }
    }
}

impl ToExpr for NumRegAB {
    fn to_expr(&self) -> vl::Expr {
        match self {
            NumRegAB::Zero => vl::Expr::new_int(0),
            NumRegAB::One => vl::Expr::new_int(1),
            NumRegAB::Two => vl::Expr::new_int(2),
        }
    }
}

impl ToInstance for Dsp {
    fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        inst.add_param("A_INPUT", self.attr.a_input.to_expr());
        inst.add_param("B_INPUT", self.attr.b_input.to_expr());
        inst.add_param("AMULTSEL", self.attr.a_multsel.to_expr());
        inst.add_param("BMULTSEL", self.attr.b_multsel.to_expr());
        inst.add_param("PREADDINSEL", self.attr.preaddinsel.to_expr());
        inst.add_param(
            "RND",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.rnd)),
        );
        inst.add_param("USE_MULT", self.attr.use_mult.to_expr());
        inst.add_param("USE_SIMD", self.attr.use_simd.to_expr());
        inst.add_param(
            "USE_WIDEXOR",
            vl::Expr::new_str(&format!("{}", self.attr.use_widexor).to_uppercase()),
        );
        inst.add_param("XORSIMD", self.attr.xorsimd.to_expr());
        inst.add_param("AUTORESET_PATDET", self.attr.autoreset_patdet.to_expr());
        inst.add_param("AUTORESET_PRIORITY", self.attr.autoreset_priority.to_expr());
        inst.add_param(
            "MASK",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.mask)),
        );
        inst.add_param(
            "PATTERN",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.pattern)),
        );
        inst.add_param("SEL_MASK", self.attr.sel_mask.to_expr());
        inst.add_param("SEL_PATTERN", self.attr.sel_pattern.to_expr());
        inst.add_param("USE_PATTERN_DETECT", self.attr.use_pattern_detect.to_expr());
        inst.add_param(
            "IS_ALUMODE_INVERTED",
            vl::Expr::new_ulit_hex(4, &format!("{:x}", self.attr.is_alumode_inverted)),
        );
        inst.add_param(
            "IS_CARRYIN_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_carryin_inverted))),
        );
        inst.add_param(
            "IS_CLK_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_clk_inverted))),
        );
        inst.add_param(
            "IS_INMODE_INVERTED",
            vl::Expr::new_ulit_hex(5, &format!("{:x}", self.attr.is_inmode_inverted)),
        );
        inst.add_param(
            "IS_OPMODE_INVERTED",
            vl::Expr::new_ulit_hex(9, &format!("{:x}", self.attr.is_opmode_inverted)),
        );
        inst.add_param(
            "IS_RSTALLCARRYIN_INVERTED",
            vl::Expr::new_ulit_bin(
                1,
                &format!("{}", u64::from(self.attr.is_rstallcarryin_inverted)),
            ),
        );
        inst.add_param(
            "IS_RSTALUMODE_INVERTED",
            vl::Expr::new_ulit_bin(
                1,
                &format!("{}", u64::from(self.attr.is_rstalumode_inverted)),
            ),
        );
        inst.add_param(
            "IS_RSTA_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rsta_inverted))),
        );
        inst.add_param(
            "IS_RSTB_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstb_inverted))),
        );
        inst.add_param(
            "IS_RSTCTRL_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstctrl_inverted))),
        );
        inst.add_param(
            "IS_RSTC_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstc_inverted))),
        );
        inst.add_param(
            "IS_RSTD_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstd_inverted))),
        );
        inst.add_param(
            "IS_RSTINMODE_INVERTED",
            vl::Expr::new_ulit_bin(
                1,
                &format!("{}", u64::from(self.attr.is_rstinmode_inverted)),
            ),
        );
        inst.add_param(
            "IS_RSTM_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstm_inverted))),
        );
        inst.add_param(
            "IS_RSTP_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstp_inverted))),
        );
        inst.add_param("ACASCREG", self.attr.acascreg.to_expr());
        inst.add_param("ADREG", self.attr.adreg.to_expr());
        inst.add_param("ALUMODEREG", self.attr.alumodereg.to_expr());
        inst.add_param("AREG", self.attr.areg.to_expr());
        inst.add_param("BCASCREG", self.attr.bcascreg.to_expr());
        inst.add_param("BREG", self.attr.breg.to_expr());
        inst.add_param("CARRYINREG", self.attr.carryinreg.to_expr());
        inst.add_param("CARRYINSELREG", self.attr.carryinselreg.to_expr());
        inst.add_param("CREG", self.attr.creg.to_expr());
        inst.add_param("DREG", self.attr.dreg.to_expr());
        inst.add_param("INMODEREG", self.attr.inmodereg.to_expr());
        inst.add_param("MREG", self.attr.mreg.to_expr());
        inst.add_param("OPMODEREG", self.attr.opmodereg.to_expr());
        inst.add_param("PREG", self.attr.preg.to_expr());
        for (k, v) in self.input.connection.iter() {
            inst.connect(&k, v.clone());
        }
        for (k, v) in self.output.connection.iter() {
            inst.connect(&k, v.clone());
        }
        if self.loc.is_placed() {
            let attr = attr_from_loc(&self.loc);
            inst.set_attr(attr);
        }
        inst
    }
    fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_input(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
        if let Some(p) = self.input.connection.get_mut(port) {
            *p = expr;
            Ok(())
        } else {
            let err = format!("input {} do not exist", port);
            Err(Error::new_xpand_error(&err))
        }
    }
    fn set_output(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
        if let Some(p) = self.output.connection.get_mut(port) {
            *p = expr;
            Ok(())
        } else {
            let err = format!("output {} do not exist", port);
            Err(Error::new_xpand_error(&err))
        }
    }
}
