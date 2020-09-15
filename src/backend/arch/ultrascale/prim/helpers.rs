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

fn lut_default_attrs() -> AttrMap {
    let mut attrs = AttrMap::new();
    attrs.insert("init".to_string(), "0".to_string());
    attrs
}

fn lut_default_outputs() -> PortMap {
    let mut outputs = PortMap::new();
    outputs.insert("y".to_string(), Expr::default());
    outputs
}

impl Lut {
    pub fn new_lut1() -> Lut {
        let mut inputs = PortMap::new();
        inputs.insert("a".to_string(), Expr::default());
        Lut {
            ty: LutTy::Lut1,
            id: String::new(),
            attrs: lut_default_attrs(),
            inputs,
            outputs: lut_default_outputs(),
            loc: None,
        }
    }

    pub fn new_lut2() -> Lut {
        let mut inputs = PortMap::new();
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        Lut {
            ty: LutTy::Lut2,
            id: String::new(),
            attrs: lut_default_attrs(),
            inputs,
            outputs: lut_default_outputs(),
            loc: None,
        }
    }

    pub fn new_lut3() -> Lut {
        let mut inputs = PortMap::new();
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("c".to_string(), Expr::default());
        Lut {
            ty: LutTy::Lut3,
            id: String::new(),
            attrs: lut_default_attrs(),
            inputs,
            outputs: lut_default_outputs(),
            loc: None,
        }
    }

    pub fn new_lut4() -> Lut {
        let mut inputs = PortMap::new();
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("c".to_string(), Expr::default());
        inputs.insert("d".to_string(), Expr::default());
        Lut {
            ty: LutTy::Lut4,
            id: String::new(),
            attrs: lut_default_attrs(),
            inputs,
            outputs: lut_default_outputs(),
            loc: None,
        }
    }

    pub fn new_lut5() -> Lut {
        let mut inputs = PortMap::new();
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("c".to_string(), Expr::default());
        inputs.insert("d".to_string(), Expr::default());
        inputs.insert("e".to_string(), Expr::default());
        Lut {
            ty: LutTy::Lut5,
            id: String::new(),
            attrs: lut_default_attrs(),
            inputs,
            outputs: lut_default_outputs(),
            loc: None,
        }
    }

    pub fn new_lut6() -> Lut {
        let mut inputs = PortMap::new();
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("c".to_string(), Expr::default());
        inputs.insert("d".to_string(), Expr::default());
        inputs.insert("e".to_string(), Expr::default());
        inputs.insert("f".to_string(), Expr::default());
        Lut {
            ty: LutTy::Lut6,
            id: String::new(),
            attrs: lut_default_attrs(),
            inputs,
            outputs: lut_default_outputs(),
            loc: None,
        }
    }

    pub fn ty(&self) -> &LutTy {
        &self.ty
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_attr(&self, attr: &str) -> String {
        if let Some(value) = self.attrs.get(attr) {
            value.to_string()
        } else {
            panic!("Error: {} attr does not exist", attr);
        }
    }

    pub fn get_input(&self, key: &str) -> &Expr {
        if let Some(input) = self.inputs.get(key) {
            input
        } else {
            panic!("Error: {} input does not exist", key);
        }
    }

    pub fn get_output(&self, key: &str) -> &Expr {
        if let Some(output) = self.outputs.get(key) {
            output
        } else {
            panic!("Error: {} output does not exist", key);
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_attr(&mut self, attr: &str, value: &str) {
        assert!(self.attrs.contains_key(attr));
        self.attrs.insert(attr.to_string(), value.to_string());
    }

    pub fn set_input(&mut self, input: &str, value: &str) {
        assert!(self.inputs.contains_key(input));
        self.inputs.insert(input.to_string(), Expr::new_ref(value));
    }

    pub fn set_input_with_index(&mut self, input: &str, value: &str, index: u32) {
        assert!(self.inputs.contains_key(input));
        self.inputs
            .insert(input.to_string(), Expr::new_index(value, index));
    }

    pub fn set_output(&mut self, output: &str, value: &str) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_ref(value));
    }

    pub fn set_output_with_index(&mut self, output: &str, value: &str, index: u32) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_index(value, index));
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
}

fn reg_default_inputs() -> PortMap {
    let mut inputs = PortMap::new();
    inputs.insert("clock".to_string(), Expr::default());
    inputs.insert("reset".to_string(), Expr::default());
    inputs.insert("a".to_string(), Expr::default());
    inputs.insert("en".to_string(), Expr::default());
    inputs
}

fn reg_default_outputs() -> PortMap {
    let mut outputs = PortMap::new();
    outputs.insert("y".to_string(), Expr::default());
    outputs
}

impl Reg {
    pub fn new_fdre() -> Reg {
        Reg {
            ty: RegTy::Fdre,
            id: String::new(),
            inputs: reg_default_inputs(),
            outputs: reg_default_outputs(),
            loc: None,
        }
    }

    pub fn new_fdse() -> Reg {
        Reg {
            ty: RegTy::Fdse,
            id: String::new(),
            inputs: reg_default_inputs(),
            outputs: reg_default_outputs(),
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

    pub fn ty(&self) -> &RegTy {
        &self.ty
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_input(&self, key: &str) -> &Expr {
        if let Some(input) = self.inputs.get(key) {
            input
        } else {
            panic!("Error: {} input does not exist", key);
        }
    }

    pub fn get_output(&self, key: &str) -> &Expr {
        if let Some(output) = self.outputs.get(key) {
            output
        } else {
            panic!("Error: {} output does not exist", key);
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_input(&mut self, input: &str, value: &str) {
        assert!(self.inputs.contains_key(input));
        self.inputs.insert(input.to_string(), Expr::new_ref(value));
    }

    pub fn set_input_with_index(&mut self, input: &str, value: &str, index: u32) {
        assert!(self.inputs.contains_key(input));
        self.inputs
            .insert(input.to_string(), Expr::new_index(value, index));
    }

    pub fn set_output(&mut self, output: &str, value: &str) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_ref(value));
    }

    pub fn set_output_with_index(&mut self, output: &str, value: &str, index: u32) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_index(value, index));
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
}

impl DspFused {
    pub fn new(op: DspFusedOp) -> DspFused {
        let mut params = ParamMap::new();
        params.insert("aw".to_string(), 30);
        params.insert("bw".to_string(), 18);
        params.insert("cw".to_string(), 48);
        params.insert("yw".to_string(), 48);
        let mut inputs = PortMap::new();
        inputs.insert("clock".to_string(), Expr::default());
        inputs.insert("reset".to_string(), Expr::default());
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("c".to_string(), Expr::default());
        match op {
            DspFusedOp::MulRegAdd => {
                inputs.insert("en_mul".to_string(), Expr::default());
            }
            DspFusedOp::RegMulAdd => {
                inputs.insert("en_input".to_string(), Expr::default());
            }
            DspFusedOp::MulAddReg => {
                inputs.insert("en_output".to_string(), Expr::default());
            }
            DspFusedOp::RegMulRegAdd => {
                inputs.insert("en_input".to_string(), Expr::default());
                inputs.insert("en_mul".to_string(), Expr::default());
            }
            DspFusedOp::MulRegAddReg => {
                inputs.insert("en_mul".to_string(), Expr::default());
                inputs.insert("en_output".to_string(), Expr::default());
            }
            DspFusedOp::RegMulAddReg => {
                inputs.insert("en_input".to_string(), Expr::default());
                inputs.insert("en_output".to_string(), Expr::default());
            }
            DspFusedOp::RegMulRegAddReg => {
                inputs.insert("en_input".to_string(), Expr::default());
                inputs.insert("en_mul".to_string(), Expr::default());
                inputs.insert("en_output".to_string(), Expr::default());
            }
            _ => (),
        }
        let mut outputs = PortMap::new();
        outputs.insert("y".to_string(), Expr::default());
        DspFused {
            op,
            id: String::new(),
            params,
            inputs,
            outputs,
        }
    }

    pub fn op(&self) -> &DspFusedOp {
        &self.op
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_param(&self, param: &str) -> i64 {
        if let Some(value) = self.params.get(param) {
            *value
        } else {
            panic!("Error: {} param does not exist", param);
        }
    }

    pub fn get_input(&self, input: &str) -> &Expr {
        if let Some(expr) = self.inputs.get(input) {
            expr
        } else {
            panic!("Error: {} input does not exist", input)
        }
    }

    pub fn get_output(&self, output: &str) -> &Expr {
        if let Some(expr) = self.outputs.get(output) {
            expr
        } else {
            panic!("Error: dsp vector output does not exist")
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_input(&mut self, input: &str, value: &str) {
        assert!(self.inputs.contains_key(input));
        self.inputs.insert(input.to_string(), Expr::new_ref(value));
    }

    pub fn set_output(&mut self, output: &str, value: &str) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_ref(value));
    }
}

impl DspVectorOp {
    pub fn is_add(&self) -> bool {
        match self {
            DspVectorOp::Add => true,
            _ => false,
        }
    }

    pub fn is_sub(&self) -> bool {
        match self {
            DspVectorOp::Sub => true,
            _ => false,
        }
    }
}

impl DspVector {
    pub fn new(op: DspVectorOp, length: u64) -> DspVector {
        let word = match length {
            1 => 48,
            2 => 24,
            3 => 12,
            4 => 12,
            _ => unimplemented!(),
        };
        let mut params = ParamMap::new();
        params.insert("width".to_string(), 48);
        params.insert("length".to_string(), length as i64);
        params.insert("word".to_string(), word as i64);
        let mut inputs = PortMap::new();
        inputs.insert("clock".to_string(), Expr::default());
        inputs.insert("reset".to_string(), Expr::default());
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("en_input".to_string(), Expr::default());
        inputs.insert("en_output".to_string(), Expr::default());
        let mut outputs = PortMap::new();
        outputs.insert("y".to_string(), Expr::default());
        DspVector {
            op,
            id: String::new(),
            params,
            inputs,
            outputs,
        }
    }

    pub fn op(&self) -> &DspVectorOp {
        &self.op
    }

    pub fn get_param(&self, param: &str) -> i64 {
        if let Some(value) = self.params.get(param) {
            *value
        } else {
            panic!("Error: {} param does not exist", param);
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_input(&self, input: &str) -> &Expr {
        if let Some(expr) = self.inputs.get(input) {
            expr
        } else {
            panic!("Error: {} input does not exist", input)
        }
    }

    pub fn get_output(&self, output: &str) -> &Expr {
        if let Some(expr) = self.outputs.get(output) {
            expr
        } else {
            panic!("Error: {} output does not exist", output)
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_input(&mut self, input: &str, value: &str) {
        assert!(self.inputs.contains_key(input));
        self.inputs.insert(input.to_string(), Expr::new_ref(value));
    }

    pub fn set_output(&mut self, output: &str, value: &str) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_ref(value));
    }
}

impl Vcc {
    pub fn new(id: &str) -> Vcc {
        let mut vcc = Vcc::default();
        vcc.set_id(id);
        vcc
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_output(&self, output: &str) -> &Expr {
        if let Some(expr) = self.outputs.get(output) {
            expr
        } else {
            panic!("Error: {} output does not exist", output);
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_output(&mut self, output: &str, value: &str) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_ref(value));
    }
}

impl Gnd {
    pub fn new(id: &str) -> Gnd {
        let mut gnd = Gnd::default();
        gnd.set_id(id);
        gnd
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_output(&self, output: &str) -> &Expr {
        if let Some(expr) = self.outputs.get(output) {
            expr
        } else {
            panic!("Error: {} output does not exist", output);
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_output(&mut self, output: &str, value: &str) {
        assert!(self.outputs.contains_key(output));
        self.outputs
            .insert(output.to_string(), Expr::new_ref(value));
    }
}

impl Const {
    pub fn new(width: u64, value: i64) -> Const {
        let mut params = ParamMap::new();
        params.insert("width".to_string(), width as i64);
        params.insert("value".to_string(), value);
        let mut inputs = PortMap::new();
        inputs.insert("gnd".to_string(), Expr::default());
        inputs.insert("vcc".to_string(), Expr::default());
        Const {
            id: String::new(),
            params,
            inputs,
        }
    }

    pub fn get_param(&self, param: &str) -> i64 {
        if let Some(value) = self.params.get(param) {
            *value
        } else {
            panic!("Error: {} param does not exist", param);
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_input(&self, input: &str) -> &Expr {
        if let Some(expr) = self.inputs.get(input) {
            expr
        } else {
            panic!("Error: {} input does not exist", input);
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_input(&mut self, input: &str, value: &str) {
        assert!(self.inputs.contains_key(input));
        self.inputs.insert(input.to_string(), Expr::new_ref(value));
    }
}
