use crate::backend::asm::ast as asm;
use crate::passes::select::instr as sel;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Expr {
    Input(String, String),
    BinOp(String, Rc<Expr>, Rc<Expr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instr {
    pub name: String,
    pub cost: u32,
    pub loc: String,
    pub ty: String,
    pub area: u32,
    pub expr: Expr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spec {
    pub isa: Vec<Instr>,
}

impl Expr {
    pub fn to_sel_instr_mut(&self, instr: &mut Vec<sel::Instr>, op_ty: sel::Ty, op_loc: sel::Loc) {
        match self {
            Expr::Input(ty, loc) => {
                let op = sel::Op::In;
                let ty = sel::Ty::from_str(ty).unwrap();
                let inp_loc = sel::Loc::from_str(loc).unwrap();
                instr.push(sel::Instr::new(op, ty, inp_loc));
            }
            Expr::BinOp(op, lhs, rhs) => {
                let op = sel::Op::from_str(op).unwrap();
                instr.push(sel::Instr::new(op, op_ty.clone(), op_loc.clone()));
                lhs.to_sel_instr_mut(instr, op_ty.clone(), op_loc.clone());
                rhs.to_sel_instr_mut(instr, op_ty.clone(), op_loc.clone());
            }
        }
    }
}

impl Instr {
    pub fn to_asm_instr(&self) -> asm::Instr {
        let ty = asm::Ty::from_str(&self.ty).unwrap();
        let loc_ty = asm::LocTy::from_str(&self.loc).unwrap();
        let asm_loc = asm::Loc {
            ty: loc_ty,
            x: asm::LocExpr::Hole,
            y: asm::LocExpr::Hole,
        };
        asm::Instr {
            ty,
            op: self.name.to_string(),
            loc: asm_loc,
            area: self.area,
            dst: None,
            params: Vec::new(),
        }
    }

    pub fn to_pattern(&self) -> sel::Pattern {
        let ty = sel::Ty::from_str(&self.ty).unwrap();
        let loc = sel::Loc::from_str(&self.loc).unwrap();
        let mut instr: Vec<sel::Instr> = Vec::new();
        self.expr.to_sel_instr_mut(&mut instr, ty, loc);
        sel::Pattern {
            name: self.name.to_string(),
            cost: self.cost,
            instr: instr.to_vec(),
        }
    }
}
