use crate::lang::ast::*;
use std::str::FromStr;

impl Ty {
    pub fn width(&self) -> u64 {
        match self {
            Ty::Bool => 1,
            Ty::UInt(w) => *w,
            Ty::SInt(w) => *w,
            Ty::Vector(d, _) => d.width(),
            _ => panic!("Error: hole does not support width"),
        }
    }

    pub fn length(&self) -> u64 {
        match self {
            Ty::Vector(_, l) => *l,
            _ => panic!("Error: type is not a vector"),
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            Ty::Vector(_, _) => true,
            _ => false,
        }
    }

    pub fn is_uint(&self) -> bool {
        match self {
            Ty::UInt(_) => true,
            _ => false,
        }
    }

    pub fn is_sint(&self) -> bool {
        match self {
            Ty::SInt(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Ty::Bool => true,
            _ => false,
        }
    }
}

impl Loc {
    pub fn new_hole() -> Loc {
        Loc::Hole
    }

    pub fn new_lut() -> Loc {
        Loc::Lut
    }

    pub fn new_dsp() -> Loc {
        Loc::Dsp
    }

    pub fn new_lum() -> Loc {
        Loc::Lum
    }

    pub fn new_ram() -> Loc {
        Loc::Ram
    }
}

impl Port {
    pub fn id(&self) -> Id {
        match self {
            Port::Input(expr) => expr.id(),
            Port::Output(expr) => expr.id(),
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Port::Input(expr) => expr.ty(),
            Port::Output(expr) => expr.ty(),
        }
    }

    pub fn expr(&self) -> &Expr {
        match self {
            Port::Input(expr) => expr,
            Port::Output(expr) => expr,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            Port::Input(_) => true,
            _ => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            Port::Output(_) => true,
            _ => false,
        }
    }
}

impl Expr {
    pub fn new_ref(id: &str, ty: Ty) -> Expr {
        Expr::Ref(id.to_string(), ty)
    }

    pub fn new_int(value: i64) -> Expr {
        Expr::Int(value)
    }

    pub fn set_id(&mut self, value: &str) {
        match self {
            Expr::Ref(id, _) => *id = value.to_string(),
            _ => panic!("Error: does not support Id"),
        }
    }

    pub fn set_ty(&mut self, value: Ty) {
        match self {
            Expr::Ref(_, ty) => *ty = value,
            _ => panic!("Error: does not support Ty"),
        }
    }

    pub fn id(&self) -> Id {
        match self {
            Expr::Ref(n, _) => n.to_string(),
            _ => panic!("Error does not support Id"),
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Expr::Ref(_, ty) => ty,
            _ => panic!("Error does not support ty"),
        }
    }

    pub fn value(&self) -> i64 {
        match self {
            Expr::Int(n) => *n,
            _ => panic!("Error does not support value"),
        }
    }
}

impl Instr {
    pub fn id(&self) -> String {
        match self {
            Instr::Std {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
            } => id.to_string(),
            Instr::Prim {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => id.to_string(),
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Instr::Std {
                id: _,
                ty,
                op: _,
                attrs: _,
                params: _,
            } => ty,
            Instr::Prim {
                id: _,
                ty,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => ty,
        }
    }

    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
            } => params,
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params,
        }
    }

    pub fn attrs(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
            } => attrs,
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
                loc: _,
            } => attrs,
        }
    }

    pub fn prim_op(&self) -> &PrimOp {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op,
                attrs: _,
                params: _,
                loc: _,
            } => op,
            _ => panic!("Error: std ops don't support prim op"),
        }
    }

    pub fn std_op(&self) -> &StdOp {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op,
                attrs: _,
                params: _,
            } => op,
            _ => panic!("Error: prim ops don't support std op"),
        }
    }

    pub fn is_prim(&self) -> bool {
        match self {
            Instr::Prim { .. } => true,
            Instr::Std { .. } => false,
        }
    }

    pub fn is_std(&self) -> bool {
        match self {
            Instr::Prim { .. } => false,
            Instr::Std { .. } => true,
        }
    }

    pub fn is_reg(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Reg,
                attrs: _,
                params: _,
                loc: _,
            } => true,
            _ => false,
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => loc,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn set_loc(&mut self, location: Loc) {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc = location,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn is_hole(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc == Loc::Hole,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn is_lut(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc == Loc::Lut,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn is_dsp(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc == Loc::Dsp,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn is_lum(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc == Loc::Lum,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn is_ram(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc == Loc::Ram,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn clear_loc(&mut self) {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => *loc = Loc::Hole,
            _ => panic!("Error: std ops don't support location"),
        }
    }

    pub fn add_param(&mut self, expr: Expr) {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
            } => params.push(expr),
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params.push(expr),
        }
    }

    pub fn clear_params(&mut self) {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
            } => params.clear(),
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params.clear(),
        }
    }
}

impl Sig {
    pub fn new(name: &str) -> Sig {
        Sig {
            id: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn inputs(&self) -> &Vec<Port> {
        &self.inputs
    }

    pub fn outputs(&self) -> &Vec<Port> {
        &self.outputs
    }

    pub fn add_input(&mut self, name: &str, ty: &str) {
        let ty = Ty::from_str(ty).unwrap();
        let expr = Expr::new_ref(name, ty);
        let port = Port::Input(expr);
        self.inputs.push(port);
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        let ty = Ty::from_str(ty).unwrap();
        let expr = Expr::new_ref(name, ty);
        let port = Port::Output(expr);
        self.outputs.push(port);
    }
}

impl Def {
    pub fn new(id: &str) -> Def {
        Def {
            sig: Sig::new(id),
            body: Vec::new(),
        }
    }

    pub fn new_with_signature(sig: Sig) -> Def {
        Def {
            sig,
            body: Vec::new(),
        }
    }

    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }

    pub fn id(&self) -> String {
        self.sig.id()
    }

    pub fn signature(&self) -> &Sig {
        &self.sig
    }

    pub fn inputs(&self) -> &Vec<Port> {
        &self.sig.inputs()
    }

    pub fn outputs(&self) -> &Vec<Port> {
        &self.sig.outputs()
    }

    pub fn add_sig(&mut self, sig: Sig) {
        self.sig = sig;
    }

    pub fn add_input(&mut self, name: &str, ty: &str) {
        self.sig.add_input(name, ty);
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        self.sig.add_output(name, ty);
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.body.push(instr);
    }
}

impl Prog {
    pub fn add_def(&mut self, def: Def) {
        self.defs.push(def);
    }

    pub fn defs(&self) -> &Vec<Def> {
        &self.defs
    }
}
