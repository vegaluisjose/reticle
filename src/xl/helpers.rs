use crate::xl::ast::*;

impl Loc {
    pub fn bel(&self) -> &Bel {
        &self.bel
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
}

impl OptVal {
    pub fn op(&self) -> Option<&OpDsp> {
        match self {
            OptVal::Op(op) => Some(op),
            _ => None,
        }
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
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg
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
    pub fn opt_op(&self) -> Option<&OpDsp> {
        if let Some(val) = self.opt.get(&Opt::Op) {
            val.op()
        } else {
            None
        }
    }
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg
    }
}

impl Instr {
    pub fn mach(&self) -> Option<&InstrMach> {
        match self {
            Instr::Mach(mach) => Some(mach),
            _ => None,
        }
    }
    pub fn basc(&self) -> Option<&InstrBasc> {
        match self {
            Instr::Basc(basc) => Some(basc),
            _ => None,
        }
    }
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
    pub fn set_dst(&mut self, dst: Expr) {
        match self {
            Instr::Basc(instr) => instr.set_dst(dst),
            Instr::Mach(instr) => instr.set_dst(dst),
        }
    }
    pub fn set_arg(&mut self, arg: Expr) {
        match self {
            Instr::Basc(instr) => instr.set_arg(arg),
            Instr::Mach(instr) => instr.set_arg(arg),
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
    pub fn body_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.body
    }
}
