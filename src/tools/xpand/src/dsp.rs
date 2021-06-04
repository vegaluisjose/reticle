use crate::errors::Error;
use crate::expr::ToExpr;
use crate::inst_name_try_from_instr;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelDsp, ExprCoord, Loc};
use crate::param::{Param, ParamMap};
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use verilog::ast as vl;
use xir::ast as xir;

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
pub enum ParamValue {
    InputTy(InputTy),
    AMultSel(AMultSel),
    BMultSel(BMultSel),
    PreAddInSel(PreAddInSel),
    UseMult(UseMult),
    UseSimd(UseSimd),
    UseWideXor(bool),
    XorSimd(XorSimd),
    AutoResetPatDet(AutoResetPatDet),
    AutoResetPriority(AutoResetPriority),
    Val(u32, u64),
    Bool(bool),
    SelMask(SelMask),
    SelPattern(SelPattern),
    UsePatternDetect(UsePatternDetect),
    // NumRegAB(NumRegAB),
    // NumReg(NumReg),
    // Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Dsp {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub param: Param<ParamValue>,
    pub input: Port,
    pub output: Port,
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

impl DefaultPort for Dsp {
    fn default_input_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("ACIN".to_string(), 30);
        width.insert("BCIN".to_string(), 18);
        width.insert("CARRYCASCIN".to_string(), 1);
        width.insert("MULTSIGNIN".to_string(), 1);
        width.insert("PCIN".to_string(), 48);
        width.insert("ALUMODE".to_string(), 4);
        width.insert("CARRYINSEL".to_string(), 3);
        width.insert("CLK".to_string(), 1);
        width.insert("INMODE".to_string(), 5);
        width.insert("OPMODE".to_string(), 9);
        width.insert("A".to_string(), 30);
        width.insert("B".to_string(), 18);
        width.insert("C".to_string(), 48);
        width.insert("CARRYIN".to_string(), 1);
        width.insert("D".to_string(), 27);
        width.insert("CEA1".to_string(), 1);
        width.insert("CEA2".to_string(), 1);
        width.insert("CEAD".to_string(), 1);
        width.insert("CEALUMODE".to_string(), 1);
        width.insert("CEB1".to_string(), 1);
        width.insert("CEB2".to_string(), 1);
        width.insert("CEC".to_string(), 1);
        width.insert("CECARRYIN".to_string(), 1);
        width.insert("CECTRL".to_string(), 1);
        width.insert("CED".to_string(), 1);
        width.insert("CEINMODE".to_string(), 1);
        width.insert("CEM".to_string(), 1);
        width.insert("CEP".to_string(), 1);
        width.insert("RSTA".to_string(), 1);
        width.insert("RSTALLCARRYIN".to_string(), 1);
        width.insert("RSTALUMODE".to_string(), 1);
        width.insert("RSTB".to_string(), 1);
        width.insert("RSTC".to_string(), 1);
        width.insert("RSTCTRL".to_string(), 1);
        width.insert("RSTD".to_string(), 1);
        width.insert("RSTINMODE".to_string(), 1);
        width.insert("RSTM".to_string(), 1);
        width.insert("RSTP".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Port { width, connection }
    }
    fn default_output_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("ACOUT".to_string(), 30);
        width.insert("BCOUT".to_string(), 18);
        width.insert("CARRYCASCOUT".to_string(), 1);
        width.insert("MULTSIGNOUT".to_string(), 1);
        width.insert("PCOUT".to_string(), 48);
        width.insert("OVERFLOW".to_string(), 1);
        width.insert("PATTERNBDETECT".to_string(), 1);
        width.insert("PATTERNDETECT".to_string(), 1);
        width.insert("UNDERFLOW".to_string(), 1);
        width.insert("CARRYOUT".to_string(), 4);
        width.insert("P".to_string(), 48);
        width.insert("XOROUT".to_string(), 8);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(""));
        }
        Port { width, connection }
    }
}

impl PartialEq for ParamValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParamValue::InputTy(_), ParamValue::InputTy(_)) => true,
            (ParamValue::AMultSel(_), ParamValue::AMultSel(_)) => true,
            (ParamValue::BMultSel(_), ParamValue::BMultSel(_)) => true,
            (ParamValue::PreAddInSel(_), ParamValue::PreAddInSel(_)) => true,
            (ParamValue::UseMult(_), ParamValue::UseMult(_)) => true,
            (ParamValue::UseSimd(_), ParamValue::UseSimd(_)) => true,
            (ParamValue::UseWideXor(_), ParamValue::UseWideXor(_)) => true,
            (ParamValue::XorSimd(_), ParamValue::XorSimd(_)) => true,
            (ParamValue::AutoResetPatDet(_), ParamValue::AutoResetPatDet(_)) => true,
            (ParamValue::AutoResetPriority(_), ParamValue::AutoResetPriority(_)) => true,
            (ParamValue::SelMask(_), ParamValue::SelMask(_)) => true,
            (ParamValue::SelPattern(_), ParamValue::SelPattern(_)) => true,
            (ParamValue::UsePatternDetect(_), ParamValue::UsePatternDetect(_)) => true,
            (ParamValue::Val(_, _), ParamValue::Val(_, _)) => true,
            (ParamValue::Bool(_), ParamValue::Bool(_)) => true,
            (_, _) => false,
        }
    }
}

impl From<InputTy> for ParamValue {
    fn from(input: InputTy) -> Self {
        ParamValue::InputTy(input)
    }
}

impl From<AMultSel> for ParamValue {
    fn from(input: AMultSel) -> Self {
        ParamValue::AMultSel(input)
    }
}

impl From<BMultSel> for ParamValue {
    fn from(input: BMultSel) -> Self {
        ParamValue::BMultSel(input)
    }
}

impl From<PreAddInSel> for ParamValue {
    fn from(input: PreAddInSel) -> Self {
        ParamValue::PreAddInSel(input)
    }
}

impl From<UseMult> for ParamValue {
    fn from(input: UseMult) -> Self {
        ParamValue::UseMult(input)
    }
}

impl From<UseSimd> for ParamValue {
    fn from(input: UseSimd) -> Self {
        ParamValue::UseSimd(input)
    }
}

impl From<XorSimd> for ParamValue {
    fn from(input: XorSimd) -> Self {
        ParamValue::XorSimd(input)
    }
}

impl From<AutoResetPatDet> for ParamValue {
    fn from(input: AutoResetPatDet) -> Self {
        ParamValue::AutoResetPatDet(input)
    }
}

impl From<AutoResetPriority> for ParamValue {
    fn from(input: AutoResetPriority) -> Self {
        ParamValue::AutoResetPriority(input)
    }
}

impl From<SelMask> for ParamValue {
    fn from(input: SelMask) -> Self {
        ParamValue::SelMask(input)
    }
}

impl From<SelPattern> for ParamValue {
    fn from(input: SelPattern) -> Self {
        ParamValue::SelPattern(input)
    }
}

impl From<UsePatternDetect> for ParamValue {
    fn from(input: UsePatternDetect) -> Self {
        ParamValue::UsePatternDetect(input)
    }
}

impl From<bool> for ParamValue {
    fn from(input: bool) -> Self {
        ParamValue::Bool(input)
    }
}

impl ToExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::InputTy(v) => v.to_expr(),
            ParamValue::AMultSel(v) => v.to_expr(),
            ParamValue::BMultSel(v) => v.to_expr(),
            ParamValue::PreAddInSel(v) => v.to_expr(),
            ParamValue::UseMult(v) => v.to_expr(),
            ParamValue::UseSimd(v) => v.to_expr(),
            ParamValue::UseWideXor(v) => {
                let s = format!("{}", v).to_uppercase();
                vl::Expr::new_str(&s)
            }
            ParamValue::XorSimd(v) => v.to_expr(),
            ParamValue::AutoResetPatDet(v) => v.to_expr(),
            ParamValue::AutoResetPriority(v) => v.to_expr(),
            ParamValue::SelMask(v) => v.to_expr(),
            ParamValue::SelPattern(v) => v.to_expr(),
            ParamValue::UsePatternDetect(v) => v.to_expr(),
            ParamValue::Val(w, v) => {
                let s = format!("{:x}", v);
                vl::Expr::new_ulit_hex(*w, &s)
            }
            ParamValue::Bool(v) => {
                let s = format!("{}", u32::from(*v));
                vl::Expr::new_ulit_bin(1, &s)
            }
        }
    }
}

//  a_input: InputTy::default(),
//  b_input: InputTy::default(),
//  a_multsel: AMultSel::default(),
//  b_multsel: BMultSel::default(),
//  preaddinsel: PreAddInSel::default(),
//  rnd: 0,
//  use_mult: UseMult::default(),
//  use_simd: UseSimd::default(),
//  use_widexor: false,
//  xorsimd: XorSimd::default(),
//  autoreset_patdet: AutoResetPatDet::default(),
//  autoreset_priority: AutoResetPriority::default(),
//  mask: u64::from_str_radix("3fffffffffff", 16).unwrap(),
//  pattern: 0,
//  sel_mask: SelMask::default(),
//  sel_pattern: SelPattern::default(),
//  use_pattern_detect: UsePatternDetect::default(),
//  is_alumode_inverted: 0,
//  is_carryin_inverted: false,
//  is_clk_inverted: false,
//  is_inmode_inverted: 0,
//  is_opmode_inverted: 0,
//  is_rstallcarryin_inverted: false,
//  is_rstalumode_inverted: false,
//  is_rsta_inverted: false,
//  is_rstb_inverted: false,
//  is_rstctrl_inverted: false,
//  is_rstc_inverted: false,
//  is_rstd_inverted: false,
//  is_rstinmode_inverted: false,
//  is_rstm_inverted: false,
//  is_rstp_inverted: false,
//  acascreg: NumRegAB::Zero,
//  adreg: NumReg::Zero,
//  alumodereg: NumReg::Zero,
//  areg: NumRegAB::Zero,
//  bcascreg: NumRegAB::Zero,
//  breg: NumRegAB::Zero,
//  carryinreg: NumReg::Zero,
//  carryinselreg: NumReg::Zero,
//  creg: NumReg::Zero,
//  dreg: NumReg::Zero,
//  inmodereg: NumReg::Zero,
//  mreg: NumReg::Zero,
//  opmodereg: NumReg::Zero,
//  preg: NumReg::Zero,

impl Default for Param<ParamValue> {
    fn default() -> Self {
        let mut map = ParamMap::new();
        map.insert("A_INPUT".to_string(), ParamValue::from(InputTy::default()));
        map.insert("B_INPUT".to_string(), ParamValue::from(InputTy::default()));
        map.insert(
            "AMULTSEL".to_string(),
            ParamValue::from(AMultSel::default()),
        );
        map.insert(
            "BMULTSEL".to_string(),
            ParamValue::from(BMultSel::default()),
        );
        map.insert(
            "PREADDINSEL".to_string(),
            ParamValue::from(PreAddInSel::default()),
        );
        map.insert("RND".to_string(), ParamValue::Val(48, 0));
        map.insert("USE_MULT".to_string(), ParamValue::from(UseMult::default()));
        map.insert("USE_SIMD".to_string(), ParamValue::from(UseSimd::default()));
        map.insert("USE_WIDEXOR".to_string(), ParamValue::UseWideXor(false));
        map.insert("XORSIMD".to_string(), ParamValue::from(XorSimd::default()));
        map.insert(
            "AUTORESET_PATDET".to_string(),
            ParamValue::from(AutoResetPatDet::default()),
        );
        map.insert(
            "AUTORESET_PRIORITY".to_string(),
            ParamValue::from(AutoResetPriority::default()),
        );
        map.insert(
            "MASK".to_string(),
            ParamValue::Val(48, u64::from_str_radix("3fffffffffff", 16).unwrap()),
        );
        map.insert("PATTERN".to_string(), ParamValue::Val(48, 0));
        map.insert("SEL_MASK".to_string(), ParamValue::from(SelMask::default()));
        map.insert(
            "SEL_PATTERN".to_string(),
            ParamValue::from(SelPattern::default()),
        );
        map.insert(
            "USE_PATTERN_DETECT".to_string(),
            ParamValue::from(UsePatternDetect::default()),
        );
        Param { map }
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
            param: Param::<ParamValue>::default(),
            input: Dsp::default_input_port(),
            output: Dsp::default_output_port(),
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
        for (k, v) in self.param.param() {
            let expr: vl::Expr = v.clone().to_expr();
            inst.add_param(k, expr);
        }
        // inst.add_param(
        //     "IS_ALUMODE_INVERTED",
        //     vl::Expr::new_ulit_hex(4, &format!("{:x}", self.attr.is_alumode_inverted)),
        // );
        // inst.add_param(
        //     "IS_CARRYIN_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_carryin_inverted))),
        // );
        // inst.add_param(
        //     "IS_CLK_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_clk_inverted))),
        // );
        // inst.add_param(
        //     "IS_INMODE_INVERTED",
        //     vl::Expr::new_ulit_hex(5, &format!("{:x}", self.attr.is_inmode_inverted)),
        // );
        // inst.add_param(
        //     "IS_OPMODE_INVERTED",
        //     vl::Expr::new_ulit_hex(9, &format!("{:x}", self.attr.is_opmode_inverted)),
        // );
        // inst.add_param(
        //     "IS_RSTALLCARRYIN_INVERTED",
        //     vl::Expr::new_ulit_bin(
        //         1,
        //         &format!("{}", u64::from(self.attr.is_rstallcarryin_inverted)),
        //     ),
        // );
        // inst.add_param(
        //     "IS_RSTALUMODE_INVERTED",
        //     vl::Expr::new_ulit_bin(
        //         1,
        //         &format!("{}", u64::from(self.attr.is_rstalumode_inverted)),
        //     ),
        // );
        // inst.add_param(
        //     "IS_RSTA_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rsta_inverted))),
        // );
        // inst.add_param(
        //     "IS_RSTB_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstb_inverted))),
        // );
        // inst.add_param(
        //     "IS_RSTCTRL_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstctrl_inverted))),
        // );
        // inst.add_param(
        //     "IS_RSTC_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstc_inverted))),
        // );
        // inst.add_param(
        //     "IS_RSTD_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstd_inverted))),
        // );
        // inst.add_param(
        //     "IS_RSTINMODE_INVERTED",
        //     vl::Expr::new_ulit_bin(
        //         1,
        //         &format!("{}", u64::from(self.attr.is_rstinmode_inverted)),
        //     ),
        // );
        // inst.add_param(
        //     "IS_RSTM_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstm_inverted))),
        // );
        // inst.add_param(
        //     "IS_RSTP_INVERTED",
        //     vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_rstp_inverted))),
        // );
        // inst.add_param("ACASCREG", self.attr.acascreg.to_expr());
        // inst.add_param("ADREG", self.attr.adreg.to_expr());
        // inst.add_param("ALUMODEREG", self.attr.alumodereg.to_expr());
        // inst.add_param("AREG", self.attr.areg.to_expr());
        // inst.add_param("BCASCREG", self.attr.bcascreg.to_expr());
        // inst.add_param("BREG", self.attr.breg.to_expr());
        // inst.add_param("CARRYINREG", self.attr.carryinreg.to_expr());
        // inst.add_param("CARRYINSELREG", self.attr.carryinselreg.to_expr());
        // inst.add_param("CREG", self.attr.creg.to_expr());
        // inst.add_param("DREG", self.attr.dreg.to_expr());
        // inst.add_param("INMODEREG", self.attr.inmodereg.to_expr());
        // inst.add_param("MREG", self.attr.mreg.to_expr());
        // inst.add_param("OPMODEREG", self.attr.opmodereg.to_expr());
        // inst.add_param("PREG", self.attr.preg.to_expr());
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

pub fn vaddrega_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut dsp = Dsp::default();
    let name = inst_name_try_from_instr(instr)?;
    dsp.set_name(&name);
    if let Some(loc) = instr.loc() {
        dsp.set_loc(loc.clone());
    }
    // let input = ["DI", "S"];
    // let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
    // for (i, e) in input.iter().zip(arg) {
    //     carry.set_input(i, e)?;
    // }
    // let input = ["CI", "CI_TOP"];
    // for i in &input {
    //     let zero = vl::Expr::new_ulit_bin(1, "0");
    //     carry.set_input(i, zero.clone())?;
    // }
    // let output = ["O"];
    // let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    // for (o, e) in output.iter().zip(dst) {
    //     carry.set_output(o, e)?;
    // }
    Ok(vec![dsp.to_stmt()])
}
