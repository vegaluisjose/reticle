use crate::asm::ast::*;

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
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
}

impl Prog {
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
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
