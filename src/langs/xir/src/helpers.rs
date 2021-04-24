use crate::ast::*;

impl OpMach {
    pub fn is_dsp(&self) -> bool {
        match self {
            OpMach::VAddRegA
            | OpMach::Mul
            | OpMach::MulAdd
            | OpMach::MulAddRegA
            | OpMach::MulAddRegACi
            | OpMach::MulAddRegACo
            | OpMach::MulAddRegACio => true,
            _ => false,
        }
    }
}

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
        self.dst = dst;
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg;
    }
}

impl InstrMach {
    pub fn op(&self) -> &OpMach {
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
    pub fn loc(&self) -> Option<&Loc> {
        self.loc.as_ref()
    }
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst;
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg;
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
    pub fn set_id(&mut self, id: &str) {
        self.sig.set_id(id);
    }
    pub fn set_sig(&mut self, sig: Sig) {
        self.sig = sig;
    }
    pub fn set_body(&mut self, body: Vec<Instr>) {
        self.body = body;
    }
    pub fn body_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.body
    }
}
