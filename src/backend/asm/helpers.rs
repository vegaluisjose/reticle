use crate::backend::asm::ast::*;

impl LocExpr {
    pub fn new_hole() -> LocExpr {
        LocExpr::Hole
    }
}

impl Loc {
    pub fn new_with_hole(ty: LocTy) -> Loc {
        Loc {
            ty,
            x: LocExpr::new_hole(),
            y: LocExpr::new_hole(),
        }
    }
}

impl Instr {
    pub fn new_std(id: &str, ty: Ty, op: StdOp) -> Instr {
        Instr::Std {
            id: id.to_string(),
            ty,
            op,
            attrs: Vec::new(),
            params: Vec::new(),
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
    pub fn is_prim(&self) -> bool {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => true,
            _ => false,
        }
    }
    pub fn prim_op(&self) -> String {
        match self {
            Instr::Prim {
                id: _,
                ty: _,
                op,
                attrs: _,
                params: _,
                loc: _,
            } => op.to_string(),
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
    pub fn set_id(&mut self, name: &str) {
        match self {
            Instr::Std {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
            } => *id = name.to_string(),
            Instr::Prim {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => *id = name.to_string(),
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
    pub fn add_attr(&mut self, expr: Expr) {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
            } => attrs.push(expr),
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
                loc: _,
            } => attrs.push(expr),
        }
    }
}

impl Prog {
    pub fn new_with_signature(sig: Sig) -> Prog {
        Prog {
            sig,
            body: Vec::new(),
        }
    }

    pub fn signature(&self) -> &Sig {
        &self.sig
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

    pub fn add_instr(&mut self, instr: Instr) {
        self.body.push(instr);
    }
}
