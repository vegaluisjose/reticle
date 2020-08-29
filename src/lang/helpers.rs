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

    pub fn is_lut(&self) -> bool {
        match self {
            Loc::Lut => true,
            _ => false,
        }
    }

    pub fn is_lum(&self) -> bool {
        match self {
            Loc::Lum => true,
            _ => false,
        }
    }

    pub fn is_dsp(&self) -> bool {
        match self {
            Loc::Dsp => true,
            _ => false,
        }
    }

    pub fn is_ram(&self) -> bool {
        match self {
            Loc::Ram => true,
            _ => false,
        }
    }

    pub fn is_hole(&self) -> bool {
        match self {
            Loc::Hole => true,
            _ => false,
        }
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

    pub fn is_ref(&self) -> bool {
        match self {
            Expr::Ref(_, _) => true,
            _ => false,
        }
    }
}

impl InstrStd {
    pub fn new(op: StdOp) -> InstrStd {
        InstrStd {
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

    pub fn clear_params(&mut self) {
        self.params.clear();
    }
}

impl InstrPrim {
    pub fn new(op: PrimOp) -> InstrPrim {
        InstrPrim {
            op,
            dst: Expr::new_ref("", Ty::Hole),
            attrs: Vec::new(),
            params: Vec::new(),
            loc: Loc::new_hole(),
        }
    }

    pub fn is_reg(&self) -> bool {
        match self.op() {
            PrimOp::Reg => true,
            _ => false,
        }
    }

    pub fn is_lut(&self) -> bool {
        self.loc.is_lut()
    }

    pub fn is_lum(&self) -> bool {
        self.loc.is_lum()
    }

    pub fn is_dsp(&self) -> bool {
        self.loc.is_dsp()
    }

    pub fn is_ram(&self) -> bool {
        self.loc.is_ram()
    }

    pub fn is_hole(&self) -> bool {
        self.loc.is_hole()
    }

    pub fn op(&self) -> &PrimOp {
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

    pub fn loc(&self) -> &Loc {
        &self.loc
    }

    pub fn dst_id(&self) -> String {
        self.dst().id()
    }

    pub fn dst_ty(&self) -> &Ty {
        &self.dst().ty()
    }

    pub fn set_op(&mut self, op: PrimOp) {
        self.op = op;
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

    pub fn clear_loc(&mut self) {
        self.loc = Loc::Hole;
    }
}

impl Instr {
    pub fn id(&self) -> String {
        match self {
            Instr::Std(instr) => instr.dst_id(),
            Instr::Prim(instr) => instr.dst_id(),
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Instr::Std(instr) => instr.dst_ty(),
            Instr::Prim(instr) => instr.dst_ty(),
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Instr::Prim(instr) => instr.loc(),
            _ => panic!("Error: not a prim instruction"),
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

    pub fn prim_op(&self) -> &PrimOp {
        match self {
            Instr::Prim(instr) => instr.op(),
            _ => panic!("Error: not a prim instruction"),
        }
    }

    pub fn std_op(&self) -> &StdOp {
        match self {
            Instr::Std(instr) => instr.op(),
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

    pub fn is_reg(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_reg(),
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

    pub fn set_loc(&mut self, loc: Loc) {
        match self {
            Instr::Prim(instr) => instr.set_loc(loc),
            _ => panic!("Error: not a prim instruction"),
        }
    }

    pub fn clear_params(&mut self) {
        match self {
            Instr::Prim(instr) => instr.clear_params(),
            Instr::Std(instr) => instr.clear_params(),
        }
    }

    pub fn clear_loc(&mut self) {
        match self {
            Instr::Prim(instr) => instr.clear_loc(),
            _ => panic!("Error: not a prim instruction"),
        }
    }

    pub fn is_lut(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_lut(),
            _ => false,
        }
    }

    pub fn is_lum(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_lum(),
            _ => false,
        }
    }

    pub fn is_dsp(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_dsp(),
            _ => false,
        }
    }

    pub fn is_ram(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_ram(),
            _ => false,
        }
    }

    pub fn is_hole(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_hole(),
            _ => false,
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
