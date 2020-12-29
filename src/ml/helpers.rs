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

impl InstrBasc {
    pub fn op(&self) -> &OpBasc {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
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
    pub fn loc(&self) -> Option<&Loc> {
        self.loc.as_ref()
    }
    pub fn opt_lookup(&self, key: &Opt) -> Option<&OptVal> {
        self.opt.get(key)
    }
}

impl Instr {
    pub fn dst(&self) -> &Expr {
        match self {
            Instr::Basc(instr) => instr.dst(),
            Instr::Mach(instr) => instr.dst(),
        }
    }
    pub fn arg(&self) -> &Expr {
        match self {
            Instr::Basc(instr) => instr.arg(),
            Instr::Mach(instr) => instr.arg(),
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
}
