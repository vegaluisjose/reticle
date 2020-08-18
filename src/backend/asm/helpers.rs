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

impl Instr {
    pub fn id(&self) -> String {
        match self {
            Instr::Std {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
            } => id.to_string(),
            Instr::Asm {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => id.to_string(),
        }
    }
    pub fn ty(&self) -> &Ty {
        match self {
            Instr::Std {
                id: _,
                ty,
                op: _,
                attrs: _,
                params: _,
            } => ty,
            Instr::Asm {
                id: _,
                ty,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => ty,
        }
    }
    pub fn attrs(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
            } => attrs,
            Instr::Asm {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
                loc: _,
            } => attrs,
        }
    }
    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
            } => params,
            Instr::Asm {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params,
        }
    }
}

// impl Prog {
//     pub fn new(prog: reticle::Prog, target: &str) -> Prog {
//         assert_eq!(target, "ultrascale", "Error: ultrascale support only");
//         Prog {
//             sig: prog.defs[0].sig.clone(),
//             body: Vec::new(),
//             target: target.to_string(),
//         }
//     }

//     pub fn id(&self) -> String {
//         self.sig.id()
//     }

//     pub fn inputs(&self) -> &Vec<Port> {
//         self.sig.inputs()
//     }

//     pub fn outputs(&self) -> &Vec<Port> {
//         self.sig.outputs()
//     }

//     pub fn add_instr(&mut self, instr: Instr) {
//         self.body.push(instr);
//     }

//     pub fn body(&self) -> &Vec<Instr> {
//         &self.body
//     }

//     pub fn to_verilog(&self) -> Module {
//         Module::from(self.clone())
//     }
// }
