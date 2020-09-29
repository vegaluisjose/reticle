use crate::asm::ast::*;

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

impl InstrPrim {
    pub fn new(loc: Loc) -> InstrPrim {
        InstrPrim {
            op: String::new(),
            dst: Expr::new_ref("", Ty::Hole),
            attrs: Vec::new(),
            params: Vec::new(),
            loc,
        }
    }

    pub fn op(&self) -> String {
        self.op.to_string()
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

    pub fn has_attrs(&self) -> bool {
        !self.attrs.is_empty()
    }

    pub fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    pub fn indexed_param(&self, index: usize) -> &Expr {
        &self.params[index]
    }

    pub fn indexed_attr(&self, index: usize) -> &Expr {
        &self.attrs[index]
    }

    pub fn loc(&self) -> &Loc {
        &self.loc
    }

    pub fn dst_id(&self) -> String {
        self.dst().id()
    }

    pub fn dst_ty(&self) -> &Ty {
        &self.dst().ty()
    }

    pub fn is_vector(&self) -> bool {
        self.dst_ty().is_vector()
    }

    pub fn is_scalar(&self) -> bool {
        self.dst_ty().is_scalar()
    }

    pub fn set_op(&mut self, op: &str) {
        self.op = op.to_string();
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
    pub fn dst_id(&self) -> String {
        match self {
            Instr::Std(instr) => instr.dst_id(),
            Instr::Prim(instr) => instr.dst_id(),
        }
    }

    pub fn dst_ty(&self) -> &Ty {
        match self {
            Instr::Std(instr) => instr.dst_ty(),
            Instr::Prim(instr) => instr.dst_ty(),
        }
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Instr::Std(instr) => instr.is_scalar(),
            Instr::Prim(instr) => instr.is_scalar(),
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            Instr::Std(instr) => instr.is_vector(),
            Instr::Prim(instr) => instr.is_vector(),
        }
    }

    pub fn dst(&self) -> &Expr {
        match self {
            Instr::Std(instr) => instr.dst(),
            Instr::Prim(instr) => instr.dst(),
        }
    }

    pub fn prim(&self) -> &InstrPrim {
        match self {
            Instr::Prim(instr) => instr,
            _ => panic!("Error: not a prim instruction"),
        }
    }

    pub fn std(&self) -> &InstrStd {
        match self {
            Instr::Std(instr) => instr,
            _ => panic!("Error: not a std instruction"),
        }
    }

    pub fn is_prim(&self) -> bool {
        match self {
            Instr::Prim(_) => true,
            _ => false,
        }
    }

    pub fn is_std(&self) -> bool {
        match self {
            Instr::Std(_) => true,
            _ => false,
        }
    }

    pub fn attrs(&self) -> &Vec<Expr> {
        match self {
            Instr::Std(instr) => instr.attrs(),
            Instr::Prim(instr) => instr.attrs(),
        }
    }

    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Instr::Std(instr) => instr.params(),
            Instr::Prim(instr) => instr.params(),
        }
    }

    pub fn indexed_param(&self, index: usize) -> &Expr {
        match self {
            Instr::Std(instr) => instr.indexed_param(index),
            Instr::Prim(instr) => instr.indexed_param(index),
        }
    }

    pub fn indexed_attr(&self, index: usize) -> &Expr {
        match self {
            Instr::Std(instr) => instr.indexed_attr(index),
            Instr::Prim(instr) => instr.indexed_attr(index),
        }
    }

    pub fn set_dst_id(&mut self, value: &str) {
        match self {
            Instr::Std(instr) => instr.set_dst_id(value),
            Instr::Prim(instr) => instr.set_dst_id(value),
        }
    }

    pub fn set_dst_ty(&mut self, ty: Ty) {
        match self {
            Instr::Std(instr) => instr.set_dst_ty(ty),
            Instr::Prim(instr) => instr.set_dst_ty(ty),
        }
    }

    pub fn set_dst(&mut self, expr: Expr) {
        match self {
            Instr::Std(instr) => instr.set_dst(expr),
            Instr::Prim(instr) => instr.set_dst(expr),
        }
    }

    pub fn add_param(&mut self, expr: Expr) {
        match self {
            Instr::Std(instr) => instr.add_param(expr),
            Instr::Prim(instr) => instr.add_param(expr),
        }
    }

    pub fn add_attr(&mut self, expr: Expr) {
        match self {
            Instr::Std(instr) => instr.add_attr(expr),
            Instr::Prim(instr) => instr.add_attr(expr),
        }
    }
}

impl Prog {
    pub fn new_with_signature(sig: Signature) -> Prog {
        Prog {
            sig,
            body: Vec::new(),
        }
    }

    pub fn signature(&self) -> &Signature {
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
