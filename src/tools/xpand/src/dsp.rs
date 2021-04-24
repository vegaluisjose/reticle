use verilog::ast as vl;

#[derive(Clone, Debug)]
pub enum Input {
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
pub enum UseWideXor {
    False,
    True,
}

#[derive(Clone, Debug)]
pub enum XorSimd {
    One,
    Two,
}

#[derive(Clone, Debug, Default)]
pub struct Attr {
    pub a_input: Input,
    pub b_input: Input,
    pub a_multsel: AMultSel,
    pub b_multsel: BMultSel,
    pub preaddinsel: PreAddInSel,
    pub rnd: u64,
    pub use_mult: UseMult,
    pub use_simd: UseSimd,
    pub use_widexor: UseWideXor,
    pub xorsimd: XorSimd,
}

#[derive(Clone, Debug)]
pub struct Dsp {
    pub name: String,
    pub prim: String,
    pub attr: Attr,
}

impl Default for Input {
    fn default() -> Self {
        Input::Direct
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

impl Default for UseWideXor {
    fn default() -> Self {
        UseWideXor::False
    }
}

impl Default for XorSimd {
    fn default() -> Self {
        XorSimd::Two
    }
}

impl Default for Dsp {
    fn default() -> Self {
        Dsp {
            name: String::new(),
            prim: "DSP48E2".to_string(),
            attr: Attr::default(),
        }
    }
}

impl Input {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            Input::Direct => vl::Expr::new_str("DIRECT"),
            Input::Cascade => vl::Expr::new_str("CASCADE"),
        }
    }
}

impl AMultSel {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            AMultSel::A => vl::Expr::new_str("A"),
            AMultSel::AD => vl::Expr::new_str("AD"),
        }
    }
}

impl BMultSel {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            BMultSel::B => vl::Expr::new_str("B"),
            BMultSel::AD => vl::Expr::new_str("AD"),
        }
    }
}

impl PreAddInSel {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            PreAddInSel::A => vl::Expr::new_str("A"),
            PreAddInSel::B => vl::Expr::new_str("B"),
        }
    }
}

impl UseMult {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            UseMult::Multiply => vl::Expr::new_str("MULTIPLY"),
            UseMult::Dynamic => vl::Expr::new_str("DYNAMIC"),
            UseMult::None => vl::Expr::new_str("NONE"),
        }
    }
}

impl UseSimd {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            UseSimd::One => vl::Expr::new_str("ONE48"),
            UseSimd::Two => vl::Expr::new_str("TWO24"),
            UseSimd::Four => vl::Expr::new_str("FOUR12"),
        }
    }
}

impl UseWideXor {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            UseWideXor::False => vl::Expr::new_str("FALSE"),
            UseWideXor::True => vl::Expr::new_str("TRUE"),
        }
    }
}

impl XorSimd {
    pub fn to_expr(self) -> vl::Expr {
        match self {
            XorSimd::One => vl::Expr::new_str("XOR12"),
            XorSimd::Two => vl::Expr::new_str("XOR24_48_96"),
        }
    }
}

impl Dsp {
    pub fn to_instance(self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        inst.connect("A_INPUT", self.attr.a_input.to_expr());
        inst.connect("B_INPUT", self.attr.b_input.to_expr());
        inst.connect("AMULTSEL", self.attr.a_multsel.to_expr());
        inst.connect("BMULTSEL", self.attr.b_multsel.to_expr());
        inst.connect("PREADDINSEL", self.attr.preaddinsel.to_expr());
        inst.connect(
            "RND",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.rnd)),
        );
        inst.connect("USE_MULT", self.attr.use_mult.to_expr());
        inst.connect("USE_SIMD", self.attr.use_simd.to_expr());
        inst.connect("USE_WIDEXOR", self.attr.use_widexor.to_expr());
        inst.connect("XORSIMD", self.attr.xorsimd.to_expr());
        inst
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
