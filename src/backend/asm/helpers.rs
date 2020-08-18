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

// use crate::backend::asm::verilog::Module;
// use crate::lang::ast as reticle;

// impl Expr {
//     pub fn new_ref(name: &str, ty: Ty) -> Expr {
//         Expr::Ref(name.to_string(), ty)
//     }

//     pub fn id(&self) -> String {
//         match self {
//             Expr::Ref(id, _) => id.to_string(),
//         }
//     }
// }

// impl Instr {
//     pub fn dst(&self) -> String {
//         self.dst.to_string()
//     }

//     pub fn params(&self) -> &Vec<Expr> {
//         &self.params
//     }

//     pub fn set_dst(&mut self, name: &str) {
//         self.dst = name.to_string();
//     }

//     pub fn add_param(&mut self, expr: Expr) {
//         self.params.push(expr);
//     }
// }

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
