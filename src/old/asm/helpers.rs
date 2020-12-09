use crate::asm::ast::*;

impl ExprCoord {
    pub fn new_hole() -> ExprCoord {
        ExprCoord::Hole
    }

    pub fn new_lit(value: u32) -> ExprCoord {
        ExprCoord::Lit(value)
    }

    pub fn is_lit(&self) -> bool {
        match self {
            ExprCoord::Lit(_) => true,
            _ => false,
        }
    }

    pub fn lit(&self) -> u32 {
        match self {
            ExprCoord::Lit(val) => *val,
            _ => panic!("Error: ExprCoord not a literal"),
        }
    }
}

impl Loc {
    pub fn new(prim: Prim) -> Loc {
        Loc {
            prim,
            x: ExprCoord::new_hole(),
            y: ExprCoord::new_hole(),
        }
    }

    pub fn new_with_xy(prim: Prim, x: u32, y: u32) -> Loc {
        Loc {
            prim,
            x: ExprCoord::new_lit(x),
            y: ExprCoord::new_lit(y),
        }
    }

    pub fn prim(&self) -> &Prim {
        &self.prim
    }

    pub fn expr_x(&self) -> &ExprCoord {
        &self.x
    }

    pub fn expr_y(&self) -> &ExprCoord {
        &self.y
    }

    pub fn lit_x(&self) -> u32 {
        self.expr_x().lit()
    }

    pub fn lit_y(&self) -> u32 {
        self.expr_y().lit()
    }

    pub fn is_valid(&self) -> bool {
        !self.prim().is_hole() & self.expr_x().is_lit() & self.expr_y().is_lit()
    }
}

impl InstrPhy {
    pub fn new(loc: Loc) -> InstrPhy {
        InstrPhy {
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

    pub fn has_valid_loc(&self) -> bool {
        self.loc().is_valid()
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

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }

    pub fn add_attr(&mut self, expr: Expr) {
        self.attrs.push(expr);
    }

    pub fn add_param(&mut self, expr: Expr) {
        self.params.push(expr);
    }

    pub fn clear_params(&mut self) {
        self.params.clear();
    }
}

impl Instr {
    pub fn dst_id(&self) -> String {
        match self {
            Instr::Std(instr) => instr.dst_id(),
            Instr::Phy(instr) => instr.dst_id(),
        }
    }

    pub fn dst_ty(&self) -> &Ty {
        match self {
            Instr::Std(instr) => instr.dst_ty(),
            Instr::Phy(instr) => instr.dst_ty(),
        }
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Instr::Std(instr) => instr.is_scalar(),
            Instr::Phy(instr) => instr.is_scalar(),
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            Instr::Std(instr) => instr.is_vector(),
            Instr::Phy(instr) => instr.is_vector(),
        }
    }

    pub fn dst(&self) -> &Expr {
        match self {
            Instr::Std(instr) => instr.dst(),
            Instr::Phy(instr) => instr.dst(),
        }
    }

    pub fn phy(&self) -> &InstrPhy {
        match self {
            Instr::Phy(instr) => instr,
            _ => panic!("Error: not a phy instruction"),
        }
    }

    pub fn std(&self) -> &InstrStd {
        match self {
            Instr::Std(instr) => instr,
            _ => panic!("Error: not a std instruction"),
        }
    }

    pub fn is_phy(&self) -> bool {
        match self {
            Instr::Phy(_) => true,
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
            Instr::Phy(instr) => instr.attrs(),
        }
    }

    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Instr::Std(instr) => instr.params(),
            Instr::Phy(instr) => instr.params(),
        }
    }

    pub fn indexed_param(&self, index: usize) -> &Expr {
        match self {
            Instr::Std(instr) => instr.indexed_param(index),
            Instr::Phy(instr) => instr.indexed_param(index),
        }
    }

    pub fn indexed_attr(&self, index: usize) -> &Expr {
        match self {
            Instr::Std(instr) => instr.indexed_attr(index),
            Instr::Phy(instr) => instr.indexed_attr(index),
        }
    }

    pub fn set_dst_id(&mut self, value: &str) {
        match self {
            Instr::Std(instr) => instr.set_dst_id(value),
            Instr::Phy(instr) => instr.set_dst_id(value),
        }
    }

    pub fn set_dst_ty(&mut self, ty: Ty) {
        match self {
            Instr::Std(instr) => instr.set_dst_ty(ty),
            Instr::Phy(instr) => instr.set_dst_ty(ty),
        }
    }

    pub fn set_dst(&mut self, expr: Expr) {
        match self {
            Instr::Std(instr) => instr.set_dst(expr),
            Instr::Phy(instr) => instr.set_dst(expr),
        }
    }

    pub fn add_param(&mut self, expr: Expr) {
        match self {
            Instr::Std(instr) => instr.add_param(expr),
            Instr::Phy(instr) => instr.add_param(expr),
        }
    }

    pub fn add_attr(&mut self, expr: Expr) {
        match self {
            Instr::Std(instr) => instr.add_attr(expr),
            Instr::Phy(instr) => instr.add_attr(expr),
        }
    }

    pub fn clear_params(&mut self) {
        match self {
            Instr::Std(instr) => instr.clear_params(),
            Instr::Phy(instr) => instr.clear_params(),
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
