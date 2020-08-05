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
}

impl Port {
    pub fn id(&self) -> Id {
        match self {
            Port::Input { id, ty: _ } => id.to_string(),
            Port::Output { id, ty: _ } => id.to_string(),
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Port::Input { id: _, ty } => ty,
            Port::Output { id: _, ty } => ty,
        }
    }
}

impl Expr {
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
        let port = Port::Input {
            id: name.to_string(),
            ty,
        };
        self.inputs.push(port);
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        let ty = Ty::from_str(ty).unwrap();
        let port = Port::Output {
            id: name.to_string(),
            ty,
        };
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
