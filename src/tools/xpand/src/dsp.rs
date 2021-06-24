use crate::errors::Error;
use crate::expr::ToExpr;
use crate::gnd::GND;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelDsp, ExprCoord, Loc};
use crate::param::Param;
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::{
    create_literal, inst_name_try_from_instr, tmp_name_try_from_term, vec_expr_try_from_expr,
    vec_expr_try_from_term,
};
use crate::{CLOCK, RESET};
use std::collections::HashSet;
use std::convert::TryFrom;
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
    NumRegAB(NumRegAB),
    NumReg(NumReg),
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
    pub fn get_input_width(&self, input: &str) -> Option<&u32> {
        self.input.get_width(input)
    }
    pub fn get_output_width(&self, output: &str) -> Option<&u32> {
        self.output.get_width(output)
    }
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
    pub fn set_param<P>(&mut self, name: &str, value: P) -> Result<(), Error>
    where
        P: Into<ParamValue>,
    {
        self.param.set_param(name, value.into())?;
        Ok(())
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
        UseMult::None
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
        let mut reset: HashSet<String> = HashSet::new();
        reset.insert("RSTA".to_string());
        reset.insert("RSTALLCARRYIN".to_string());
        reset.insert("RSTALUMODE".to_string());
        reset.insert("RSTB".to_string());
        reset.insert("RSTC".to_string());
        reset.insert("RSTCTRL".to_string());
        reset.insert("RSTD".to_string());
        reset.insert("RSTINMODE".to_string());
        reset.insert("RSTM".to_string());
        reset.insert("RSTP".to_string());
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
        for r in reset.iter() {
            width.insert(r.to_string(), 1);
        }
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            let expr = if k == "CLK" {
                vl::Expr::new_ref(CLOCK)
            } else if reset.contains(k) {
                vl::Expr::new_ref(RESET)
            } else {
                create_literal(*v as u64, 0)
            };
            connection.insert(k.clone(), expr);
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
            (ParamValue::NumRegAB(_), ParamValue::NumRegAB(_)) => true,
            (ParamValue::NumReg(_), ParamValue::NumReg(_)) => true,
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

impl From<NumRegAB> for ParamValue {
    fn from(input: NumRegAB) -> Self {
        ParamValue::NumRegAB(input)
    }
}

impl From<NumReg> for ParamValue {
    fn from(input: NumReg) -> Self {
        ParamValue::NumReg(input)
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
            ParamValue::NumRegAB(v) => v.to_expr(),
            ParamValue::NumReg(v) => v.to_expr(),
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

impl Default for Param<ParamValue> {
    fn default() -> Self {
        let mut param = Param::<ParamValue>::new();
        param.insert("A_INPUT".to_string(), ParamValue::from(InputTy::default()));
        param.insert("B_INPUT".to_string(), ParamValue::from(InputTy::default()));
        param.insert(
            "AMULTSEL".to_string(),
            ParamValue::from(AMultSel::default()),
        );
        param.insert(
            "BMULTSEL".to_string(),
            ParamValue::from(BMultSel::default()),
        );
        param.insert(
            "PREADDINSEL".to_string(),
            ParamValue::from(PreAddInSel::default()),
        );
        param.insert("RND".to_string(), ParamValue::Val(48, 0));
        param.insert("USE_MULT".to_string(), ParamValue::from(UseMult::default()));
        param.insert("USE_SIMD".to_string(), ParamValue::from(UseSimd::default()));
        param.insert("USE_WIDEXOR".to_string(), ParamValue::UseWideXor(false));
        param.insert("XORSIMD".to_string(), ParamValue::from(XorSimd::default()));
        param.insert(
            "AUTORESET_PATDET".to_string(),
            ParamValue::from(AutoResetPatDet::default()),
        );
        param.insert(
            "AUTORESET_PRIORITY".to_string(),
            ParamValue::from(AutoResetPriority::default()),
        );
        param.insert(
            "MASK".to_string(),
            ParamValue::Val(48, u64::from_str_radix("3fffffffffff", 16).unwrap()),
        );
        param.insert("PATTERN".to_string(), ParamValue::Val(48, 0));
        param.insert("SEL_MASK".to_string(), ParamValue::from(SelMask::default()));
        param.insert(
            "SEL_PATTERN".to_string(),
            ParamValue::from(SelPattern::default()),
        );
        param.insert("IS_ALUMODE_INVERTED".to_string(), ParamValue::Val(4, 0));
        param.insert("IS_CARRYIN_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_CLK_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_INMODE_INVERTED".to_string(), ParamValue::Val(5, 0));
        param.insert("IS_OPMODE_INVERTED".to_string(), ParamValue::Val(9, 0));
        param.insert(
            "IS_RSTALLCARRYIN_INVERTED".to_string(),
            ParamValue::Bool(false),
        );
        param.insert(
            "IS_RSTALUMODE_INVERTED".to_string(),
            ParamValue::Bool(false),
        );
        param.insert("IS_RSTA_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTB_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTCTRL_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTC_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTD_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTINMODE_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTM_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert("IS_RSTP_INVERTED".to_string(), ParamValue::Bool(false));
        param.insert(
            "ACASCREG".to_string(),
            ParamValue::from(NumRegAB::default()),
        );
        param.insert("ADREG".to_string(), ParamValue::from(NumReg::default()));
        param.insert(
            "ALUMODEREG".to_string(),
            ParamValue::from(NumReg::default()),
        );
        param.insert("AREG".to_string(), ParamValue::from(NumRegAB::default()));
        param.insert(
            "BCASCREG".to_string(),
            ParamValue::from(NumRegAB::default()),
        );
        param.insert("BREG".to_string(), ParamValue::from(NumRegAB::default()));
        param.insert(
            "CARRYINREG".to_string(),
            ParamValue::from(NumReg::default()),
        );
        param.insert(
            "CARRYINSELREG".to_string(),
            ParamValue::from(NumReg::default()),
        );
        param.insert("CREG".to_string(), ParamValue::from(NumReg::default()));
        param.insert("DREG".to_string(), ParamValue::from(NumReg::default()));
        param.insert("INMODEREG".to_string(), ParamValue::from(NumReg::default()));
        param.insert("MREG".to_string(), ParamValue::from(NumReg::default()));
        param.insert("OPMODEREG".to_string(), ParamValue::from(NumReg::default()));
        param.insert("PREG".to_string(), ParamValue::from(NumReg::default()));
        param
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

fn simd_opt_try_from_term(term: &xir::ExprTerm) -> Result<ParamValue, Error> {
    if let Some(length) = term.length() {
        match length {
            4 => Ok(ParamValue::from(UseSimd::Four)),
            3 => Ok(ParamValue::from(UseSimd::Four)),
            2 => Ok(ParamValue::from(UseSimd::Two)),
            1 => Ok(ParamValue::from(UseSimd::One)),
            _ => Err(Error::new_xpand_error("unsupported length")),
        }
    } else {
        Err(Error::new_xpand_error("it must be a vector type"))
    }
}

fn vec_word_width_try_from_term(term: &xir::ExprTerm) -> Result<i32, Error> {
    if let Some(length) = term.length() {
        match length {
            4 => Ok(12),
            3 => Ok(12),
            2 => Ok(24),
            1 => Ok(48),
            _ => Err(Error::new_xpand_error("unsupported length")),
        }
    } else {
        Ok(48)
    }
}

fn vl_expr_try_from_term(term: &xir::ExprTerm, lsb: usize, msb: usize) -> Result<vl::Expr, Error> {
    let word_width = vec_word_width_try_from_term(term)?;
    let expr: Vec<vl::Expr> = vec_expr_try_from_term(term)?;
    let mut bits: Vec<vl::Expr> = Vec::new();
    if let Some(width) = term.width() {
        for e in expr {
            if let Ok(wbits) = i32::try_from(width) {
                let width = i32::try_from(width).unwrap(); // FIXME: use better errors
                for i in 0..wbits {
                    bits.push(vl::Expr::new_index_bit(&e.id(), i));
                }
                let pbits = word_width - width;
                for _ in 0..pbits {
                    bits.push(vl::Expr::new_ref(GND));
                }
            }
        }
        let mut cat = vl::ExprConcat::default();
        for i in bits.iter().take(msb + 1).skip(lsb) {
            cat.add_expr(i.clone());
        }
        Ok(vl::Expr::from(cat))
    } else {
        Err(Error::new_xpand_error("term must be var"))
    }
}

pub fn vaddrega_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut dsp = Dsp::default();
    let mut stmt: Vec<vl::Stmt> = Vec::new();
    let name = inst_name_try_from_instr(instr)?;
    dsp.set_name(&name);
    // loc
    if let Some(loc) = instr.loc() {
        dsp.set_loc(loc.clone());
    }
    // simd
    if let Some(t) = instr.dst().term() {
        dsp.set_param("USE_SIMD", simd_opt_try_from_term(t)?)?;
    }
    // registers
    dsp.set_param("CREG", ParamValue::from(NumReg::One))?;
    dsp.set_param("AREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("BREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("ACASCREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("BCASCREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("PREG", ParamValue::from(NumReg::One))?;
    // opcode
    dsp.set_input("OPMODE", create_literal(9, 51))?;
    // input
    let left_term = instr.arg().get_term(0)?;
    let c_msb = dsp.get_input_width("C").unwrap() - 1;
    let c_expr = vl_expr_try_from_term(left_term, 0, c_msb as usize)?;
    dsp.set_input("C", c_expr)?;
    let right_term = instr.arg().get_term(1)?;
    let b_width = *dsp.get_input_width("B").unwrap();
    let b_expr = vl_expr_try_from_term(right_term, 0, (b_width - 1) as usize)?;
    dsp.set_input("B", b_expr)?;
    let a_width = dsp.get_input_width("A").unwrap();
    let a_expr = vl_expr_try_from_term(
        right_term,
        b_width as usize,
        (b_width + a_width - 1) as usize,
    )?;
    dsp.set_input("A", a_expr)?;
    let l_en_term = instr.arg().get_term(2)?;
    let r_en_term = instr.arg().get_term(3)?;
    let o_en_term = instr.arg().get_term(4)?;
    let l_en_name = String::try_from(l_en_term.clone())?;
    let r_en_name = String::try_from(r_en_term.clone())?;
    let o_en_name = String::try_from(o_en_term.clone())?;
    dsp.set_input("CEC", vl::Expr::new_ref(&l_en_name))?;
    dsp.set_input("CEA1", vl::Expr::new_ref(&r_en_name))?;
    dsp.set_input("CEA2", vl::Expr::new_ref(&r_en_name))?;
    dsp.set_input("CEB1", vl::Expr::new_ref(&r_en_name))?;
    dsp.set_input("CEB2", vl::Expr::new_ref(&r_en_name))?;
    dsp.set_input("CEP", vl::Expr::new_ref(&o_en_name))?;
    // output
    let dst_term = instr.dst().get_term(0)?;
    let output = tmp_name_try_from_term(dst_term)?;
    dsp.set_output("P", vl::Expr::new_ref(&output))?;
    stmt.push(dsp.to_stmt());
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let wbits = vec_word_width_try_from_term(dst_term)?;
    if let Some(width) = dst_term.width() {
        if let Ok(ebits) = i32::try_from(width) {
            for (i, e) in dst.iter().enumerate() {
                let i = i as i32;
                let hi = i * wbits + (ebits - 1);
                let lo = i * wbits;
                let assign = vl::Parallel::Assign(
                    e.clone(),
                    vl::Expr::new_slice(&output, vl::Expr::new_int(hi), vl::Expr::new_int(lo)),
                );
                stmt.push(vl::Stmt::from(assign));
            }
        }
    }
    Ok(stmt)
}

pub fn muladdrega_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut dsp = Dsp::default();
    let mut stmt: Vec<vl::Stmt> = Vec::new();
    let name = inst_name_try_from_instr(instr)?;
    dsp.set_name(&name);
    // loc
    if let Some(loc) = instr.loc() {
        dsp.set_loc(loc.clone());
    }
    // multiply
    dsp.set_param("USE_MULT", ParamValue::from(UseMult::Multiply))?;
    // registers
    dsp.set_param("AREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("BREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("ACASCREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("BCASCREG", ParamValue::from(NumRegAB::One))?;
    dsp.set_param("MREG", ParamValue::from(NumReg::One))?;
    dsp.set_param("PREG", ParamValue::from(NumReg::One))?;
    // opcode
    dsp.set_input("OPMODE", create_literal(9, 53))?;
    // input
    let a_term = instr.arg().get_term(0)?;
    let a_width = *dsp.get_input_width("A").unwrap();
    let a_expr = vl_expr_try_from_term(a_term, 0, (a_width - 1) as usize)?;
    dsp.set_input("A", a_expr)?;
    let b_term = instr.arg().get_term(1)?;
    let b_width = *dsp.get_input_width("B").unwrap();
    let b_expr = vl_expr_try_from_term(b_term, 0, (b_width - 1) as usize)?;
    dsp.set_input("B", b_expr)?;
    let c_term = instr.arg().get_term(2)?;
    let c_width = *dsp.get_input_width("C").unwrap();
    let c_expr = vl_expr_try_from_term(c_term, 0, (c_width - 1) as usize)?;
    dsp.set_input("C", c_expr)?;
    let ena_term = instr.arg().get_term(3)?;
    let enb_term = instr.arg().get_term(4)?;
    let enm_term = instr.arg().get_term(5)?;
    let enp_term = instr.arg().get_term(6)?;
    let ena_name = String::try_from(ena_term.clone())?;
    let enb_name = String::try_from(enb_term.clone())?;
    let enm_name = String::try_from(enm_term.clone())?;
    let enp_name = String::try_from(enp_term.clone())?;
    dsp.set_input("CEA1", vl::Expr::new_ref(&ena_name))?;
    dsp.set_input("CEA2", vl::Expr::new_ref(&ena_name))?;
    dsp.set_input("CEB1", vl::Expr::new_ref(&enb_name))?;
    dsp.set_input("CEB2", vl::Expr::new_ref(&enb_name))?;
    dsp.set_input("CEM", vl::Expr::new_ref(&enm_name))?;
    dsp.set_input("CEP", vl::Expr::new_ref(&enp_name))?;
    // output
    let dst_term = instr.dst().get_term(0)?;
    let output = tmp_name_try_from_term(dst_term)?;
    dsp.set_output("P", vl::Expr::new_ref(&output))?;
    stmt.push(dsp.to_stmt());
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let wbits = vec_word_width_try_from_term(dst_term)?;
    if let Some(width) = dst_term.width() {
        if let Ok(ebits) = i32::try_from(width) {
            for (i, e) in dst.iter().enumerate() {
                let i = i as i32;
                let hi = i * wbits + (ebits - 1);
                let lo = i * wbits;
                let assign = vl::Parallel::Assign(
                    e.clone(),
                    vl::Expr::new_slice(&output, vl::Expr::new_int(hi), vl::Expr::new_int(lo)),
                );
                stmt.push(vl::Stmt::from(assign));
            }
        }
    }
    Ok(stmt)
}
