use crate::backend::asm::ast::*;
use crate::backend::target::descriptor::*;
use crate::backend::target::spec::*;
use std::str::FromStr;

impl From<SpecInstr> for Instr {
    fn from(spec_instr: SpecInstr) -> Self {
        Instr::Asm {
            id: String::new(),
            ty: Ty::from_str(&spec_instr.ty()).unwrap(),
            op: spec_instr.name(),
            attrs: Vec::new(),
            params: Vec::new(),
            loc: Loc::new_with_hole(LocTy::from_str(&spec_instr.loc()).unwrap()),
        }
    }
}

impl From<SpecInstr> for Tile {
    fn from(spec_instr: SpecInstr) -> Self {
        Tile {
            instr: Instr::from(spec_instr),
        }
    }
}

impl From<Spec> for Descriptor {
    fn from(spec: Spec) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for instr in spec.isa.iter() {
            tiles.push(Tile::from(instr.clone()));
        }
        Descriptor {
            tiles: tiles.to_vec(),
        }
    }
}

// impl Expr {
//     // pub fn to_sel_instr_mut(&self, instr: &mut Vec<sel::Instr>, op_ty: sel::Ty, op_loc: sel::Loc) {
//     //     match self {
//     //         Expr::Input(ty, loc) => {
//     //             let op = sel::Op::In;
//     //             let ty = sel::Ty::from_str(ty).unwrap();
//     //             let inp_loc = sel::Loc::from_str(loc).unwrap();
//     //             instr.push(sel::Instr::new(op, ty, inp_loc));
//     //         }
//     //         Expr::BinOp(op, lhs, rhs) => {
//     //             let op = sel::Op::from_str(op).unwrap();
//     //             instr.push(sel::Instr::new(op, op_ty.clone(), op_loc.clone()));
//     //             lhs.to_sel_instr_mut(instr, op_ty.clone(), op_loc.clone());
//     //             rhs.to_sel_instr_mut(instr, op_ty, op_loc);
//     //         }
//     //     }
//     // }
// }
