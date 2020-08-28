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

impl StdInstr {
    pub fn new(op: StdOp) -> StdInstr {
        StdInstr {
            op,
            dst: Expr::new_ref("", Ty::Hole),
            attrs: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn op(&self) -> &StdOp {
        &self.op
    }

    pub fn dst(&self) -> &Expr {
        &self.dst
    }

    pub fn attrs(&self) -> &Vec<Expr> {
        &self.attrs
    }

    pub fn params(&self) -> &Vec<Expr> {
        &self.params
    }

    pub fn dst_id(&self) -> String {
        self.dst().id()
    }

    pub fn dst_ty(&self) -> &Ty {
        &self.dst().ty()
    }

    pub fn set_dst_id(&mut self, id: &str) {
        self.dst.set_id(id);
    }

    pub fn set_dst_ty(&mut self, ty: Ty) {
        self.dst.set_ty(ty);
    }

    pub fn set_dst(&mut self, expr: Expr) {
        self.dst = expr;
    }

    pub fn add_attr(&mut self, expr: Expr) {
        self.attrs.push(expr);
    }

    pub fn add_param(&mut self, expr: Expr) {
        self.params.push(expr);
    }
}

impl Instr {
    pub fn new_std(id: &str, ty: Ty, op: StdOp) -> Instr {
        Instr::Std {
            op,
            dst: Expr::new_ref(id, ty),
            attrs: Vec::new(),
            params: Vec::new(),
        }
    }
    pub fn id(&self) -> String {
        match self {
            Instr::Std {
                op: _,
                dst,
                attrs: _,
                params: _,
            } => dst.id(),
            Instr::Prim {
                op: _,
                dst,
                attrs: _,
                params: _,
                loc: _,
            } => dst.id(),
        }
    }
    pub fn ty(&self) -> &Ty {
        match self {
            Instr::Std {
                op: _,
                dst,
                attrs: _,
                params: _,
            } => dst.ty(),
            Instr::Prim {
                op: _,
                dst,
                attrs: _,
                params: _,
                loc: _,
            } => dst.ty(),
        }
    }
    pub fn is_prim(&self) -> bool {
        match self {
            Instr::Prim { .. } => true,
            _ => false,
        }
    }
    pub fn prim_op(&self) -> String {
        match self {
            Instr::Prim {
                op,
                dst: _,
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
                op,
                dst: _,
                attrs: _,
                params: _,
            } => op,
            _ => panic!("Error: prim ops don't support std op"),
        }
    }
    pub fn attrs(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                op: _,
                dst: _,
                attrs,
                params: _,
            } => attrs,
            Instr::Prim {
                op: _,
                dst: _,
                attrs,
                params: _,
                loc: _,
            } => attrs,
        }
    }
    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                op: _,
                dst: _,
                attrs: _,
                params,
            } => params,
            Instr::Prim {
                op: _,
                dst: _,
                attrs: _,
                params,
                loc: _,
            } => params,
        }
    }
    pub fn set_id(&mut self, value: &str) {
        match self {
            Instr::Std {
                op: _,
                dst,
                attrs: _,
                params: _,
            } => dst.set_id(value),
            Instr::Prim {
                op: _,
                dst,
                attrs: _,
                params: _,
                loc: _,
            } => dst.set_id(value),
        }
    }
    pub fn add_param(&mut self, expr: Expr) {
        match self {
            Instr::Std {
                op: _,
                dst: _,
                attrs: _,
                params,
            } => params.push(expr),
            Instr::Prim {
                op: _,
                dst: _,
                attrs: _,
                params,
                loc: _,
            } => params.push(expr),
        }
    }
    pub fn add_attr(&mut self, expr: Expr) {
        match self {
            Instr::Std {
                op: _,
                dst: _,
                attrs,
                params: _,
            } => attrs.push(expr),
            Instr::Prim {
                op: _,
                dst: _,
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
