use crate::ml::ast::*;

impl Loc {
    pub fn bel(&self) -> Option<&Bel> {
        self.bel.as_ref()
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
}

impl InstrMach {
    pub fn op(&self) -> &OpMach {
        &self.op
    }
    pub fn opt(&self) -> &OptMap {
        &self.opt
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}

impl Prog {
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
}
