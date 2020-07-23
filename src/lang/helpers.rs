use crate::lang::ast::*;
use std::str::FromStr;

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
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Expr::Ref(_, ty) => ty,
        }
    }
}

impl Instr {
    pub fn new_with_args(
        dst: &str,
        op_ty: &str,
        op: &str,
        lhs: &str,
        lhs_ty: &str,
        rhs: &str,
        rhs_ty: &str,
        loc: &str,
    ) -> Instr {
        let op_ty = Ty::from_str(op_ty).unwrap();
        let lhs_ty = Ty::from_str(lhs_ty).unwrap();
        let rhs_ty = Ty::from_str(rhs_ty).unwrap();
        Instr::Prim {
            id: dst.to_string(),
            ty: op_ty,
            op: PrimOp::from_str(op).unwrap(),
            attrs: vec![],
            params: vec![
                Expr::Ref(lhs.to_string(), lhs_ty),
                Expr::Ref(rhs.to_string(), rhs_ty),
            ],
            loc: Loc::from_str(loc).unwrap(),
        }
    }

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
    pub fn new(name: &str) -> Def {
        Def {
            sig: Sig::new(name),
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

    pub fn inputs(&self) -> &Vec<Port> {
        &self.sig.inputs()
    }

    pub fn outputs(&self) -> &Vec<Port> {
        &self.sig.outputs()
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
