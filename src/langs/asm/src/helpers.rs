use crate::ast::*;

impl Loc {
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
    pub fn is_lut(&self) -> bool {
        matches!(self.prim, Prim::Lut)
    }
    pub fn is_dsp(&self) -> bool {
        matches!(self.prim, Prim::Dsp)
    }
    pub fn is_unplaced(&self) -> bool {
        match (self.x(), self.y()) {
            (ExprCoord::Val(_), ExprCoord::Val(_)) => false,
            (_, _) => true,
        }
    }
    pub fn set_x(&mut self, x: ExprCoord) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: ExprCoord) {
        self.y = y;
    }
}

impl OpAsm {
    pub fn name(&self) -> String {
        match self {
            OpAsm::Op(n) => n.to_string(),
        }
    }
}

impl InstrAsm {
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn op(&self) -> &OpAsm {
        &self.op
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
    pub fn is_lut(&self) -> bool {
        self.loc.is_lut()
    }
    pub fn is_dsp(&self) -> bool {
        self.loc.is_dsp()
    }
    pub fn set_op(&mut self, op: OpAsm) {
        self.op = op;
    }
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst;
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg;
    }
}

impl Instr {
    pub fn dst(&self) -> &Expr {
        match self {
            Instr::Asm(instr) => instr.dst(),
            Instr::Wire(instr) => instr.dst(),
        }
    }
    pub fn arg(&self) -> &Expr {
        match self {
            Instr::Asm(instr) => instr.arg(),
            Instr::Wire(instr) => instr.arg(),
        }
    }
    pub fn set_dst(&mut self, dst: Expr) {
        match self {
            Instr::Asm(instr) => instr.set_dst(dst),
            Instr::Wire(instr) => instr.set_dst(dst),
        }
    }
    pub fn set_arg(&mut self, arg: Expr) {
        match self {
            Instr::Asm(instr) => instr.set_arg(arg),
            Instr::Wire(instr) => instr.set_arg(arg),
        }
    }
}

impl Prog {
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
    pub fn set_id(&mut self, id: &str) {
        self.sig.set_id(id);
    }
    pub fn set_sig(&mut self, sig: Sig) {
        self.sig = sig;
    }
    pub fn set_body(&mut self, instr: Vec<Instr>) {
        self.body = instr;
    }
    pub fn body_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.body
    }
}
