use crate::backend::arch::ultrascale::prim::ast::*;

impl Expr {
    pub fn id(&self) -> String {
        match self {
            Expr::Ref(name) => name.to_string(),
            Expr::Index(name, _) => name.to_string(),
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            Expr::Ref(name) => name.is_empty(),
            _ => false,
        }
    }

    pub fn new_ref(name: &str) -> Expr {
        Expr::Ref(name.to_string())
    }

    pub fn new_index(name: &str, index: u32) -> Expr {
        Expr::Index(name.to_string(), index)
    }
}

impl Lut {
    pub fn new_lut2() -> Lut {
        Lut {
            ty: LutTy::Lut2,
            id: String::new(),
            init: "0".to_string(),
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut3() -> Lut {
        Lut {
            ty: LutTy::Lut3,
            id: String::new(),
            init: "0".to_string(),
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut4() -> Lut {
        Lut {
            ty: LutTy::Lut4,
            id: String::new(),
            init: "0".to_string(),
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut5() -> Lut {
        Lut {
            ty: LutTy::Lut5,
            id: String::new(),
            init: "0".to_string(),
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut6() -> Lut {
        Lut {
            ty: LutTy::Lut6,
            id: String::new(),
            init: "0".to_string(),
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn ty(&self) -> &LutTy {
        &self.ty
    }

    pub fn init(&self) -> String {
        self.init.to_string()
    }

    pub fn inputs(&self) -> &Vec<Expr> {
        &self.inputs
    }

    pub fn output(&self) -> &Expr {
        &self.output
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_init(&mut self, value: &str) {
        self.init = value.to_string();
    }

    pub fn add_input(&mut self, name: &str) {
        self.inputs.push(Expr::new_ref(name));
    }

    pub fn add_input_with_index(&mut self, name: &str, index: u32) {
        self.inputs.push(Expr::new_index(name, index));
    }

    pub fn set_output(&mut self, name: &str) {
        self.output = Expr::new_ref(name);
    }

    pub fn set_output_with_index(&mut self, name: &str, index: u32) {
        self.output = Expr::new_index(name, index);
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
}

impl Reg {
    pub fn new_fdre() -> Reg {
        Reg {
            ty: RegTy::Fdre,
            id: String::new(),
            clock: Expr::default(),
            reset: Expr::default(),
            en: Expr::default(),
            input: Expr::default(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_fdse() -> Reg {
        Reg {
            ty: RegTy::Fdse,
            id: String::new(),
            clock: Expr::default(),
            reset: Expr::default(),
            en: Expr::default(),
            input: Expr::default(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn is_fdre(&self) -> bool {
        match self.ty {
            RegTy::Fdre => true,
            _ => false,
        }
    }

    pub fn is_fdse(&self) -> bool {
        match self.ty {
            RegTy::Fdse => true,
            _ => false,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn ty(&self) -> &RegTy {
        &self.ty
    }

    pub fn clock(&self) -> &Expr {
        &self.clock
    }

    pub fn reset(&self) -> &Expr {
        &self.reset
    }

    pub fn en(&self) -> &Expr {
        &self.en
    }

    pub fn input(&self) -> &Expr {
        &self.input
    }

    pub fn output(&self) -> &Expr {
        &self.output
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_clock(&mut self, clock: &str) {
        self.clock = Expr::new_ref(clock);
    }

    pub fn set_reset(&mut self, reset: &str) {
        self.reset = Expr::new_ref(reset);
    }

    pub fn set_en(&mut self, en: &str) {
        self.en = Expr::new_ref(en);
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = Expr::new_ref(input);
    }

    pub fn set_input_with_index(&mut self, input: &str, index: u32) {
        self.input = Expr::new_index(input, index);
    }

    pub fn set_output(&mut self, output: &str) {
        self.output = Expr::new_ref(output);
    }

    pub fn set_output_with_index(&mut self, output: &str, index: u32) {
        self.output = Expr::new_index(output, index);
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
}

impl DspOp {
    pub fn is_add(&self) -> bool {
        match self {
            DspOp::Add => true,
            _ => false,
        }
    }

    pub fn is_sub(&self) -> bool {
        match self {
            DspOp::Sub => true,
            _ => false,
        }
    }

    pub fn is_mul(&self) -> bool {
        match self {
            DspOp::Mul => true,
            _ => false,
        }
    }
}

impl DspTy {
    pub fn is_scalar(&self) -> bool {
        match self {
            DspTy::Scalar => true,
            _ => false,
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            DspTy::Vector(_) => true,
            _ => false,
        }
    }

    pub fn length(&self) -> u64 {
        match self {
            DspTy::Vector(l) => *l,
            _ => panic!("Error: scalar does not support length"),
        }
    }
}

impl Dsp {
    pub fn new_scalar(op: DspOp) -> Dsp {
        Dsp {
            op,
            ty: DspTy::Scalar,
            width: 48,
            id: String::new(),
            clock: Expr::default(),
            reset: Expr::default(),
            en: Expr::default(),
            left: Expr::default(),
            right: Expr::default(),
            output: Expr::default(),
        }
    }
    pub fn new_vector(op: DspOp, length: u64) -> Dsp {
        Dsp {
            op,
            ty: DspTy::Vector(length),
            width: 48,
            id: String::new(),
            clock: Expr::default(),
            reset: Expr::default(),
            en: Expr::default(),
            left: Expr::default(),
            right: Expr::default(),
            output: Expr::default(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn op(&self) -> &DspOp {
        &self.op
    }

    pub fn ty(&self) -> &DspTy {
        &self.ty
    }

    pub fn clock(&self) -> &Expr {
        &self.clock
    }

    pub fn reset(&self) -> &Expr {
        &self.reset
    }

    pub fn en(&self) -> &Expr {
        &self.en
    }

    pub fn left(&self) -> &Expr {
        &self.left
    }

    pub fn right(&self) -> &Expr {
        &self.right
    }

    pub fn output(&self) -> &Expr {
        &self.output
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn length(&self) -> u64 {
        self.ty.length()
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_clock(&mut self, clock: &str) {
        self.clock = Expr::new_ref(clock);
    }

    pub fn set_reset(&mut self, reset: &str) {
        self.reset = Expr::new_ref(reset);
    }

    pub fn set_en(&mut self, en: &str) {
        self.en = Expr::new_ref(en);
    }

    pub fn set_left(&mut self, left: &str) {
        self.left = Expr::new_ref(left);
    }

    pub fn set_right(&mut self, right: &str) {
        self.right = Expr::new_ref(right);
    }

    pub fn set_output(&mut self, output: &str) {
        self.output = Expr::new_ref(output);
    }
}

impl Vcc {
    pub fn new(id: &str) -> Vcc {
        Vcc {
            id: id.to_string(),
            output: Expr::default(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn output(&self) -> &Expr {
        &self.output
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_output(&mut self, output: &str) {
        self.output = Expr::new_ref(output);
    }
}

impl Gnd {
    pub fn new(id: &str) -> Gnd {
        Gnd {
            id: id.to_string(),
            output: Expr::default(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn output(&self) -> &Expr {
        &self.output
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_output(&mut self, output: &str) {
        self.output = Expr::new_ref(output);
    }
}

impl Const {
    pub fn new(width: u64, value: i64) -> Const {
        Const {
            id: String::new(),
            gnd: Expr::default(),
            vcc: Expr::default(),
            width,
            value,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn gnd(&self) -> &Expr {
        &self.gnd
    }

    pub fn vcc(&self) -> &Expr {
        &self.vcc
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_gnd(&mut self, gnd: &str) {
        self.gnd = Expr::new_ref(gnd);
    }

    pub fn set_vcc(&mut self, vcc: &str) {
        self.vcc = Expr::new_ref(vcc);
    }
}
