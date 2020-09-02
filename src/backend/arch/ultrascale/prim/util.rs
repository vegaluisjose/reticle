use crate::backend::arch::ultrascale::prim::ast::*;

pub fn regs_from_init(width: u64, init: i64) -> Vec<Reg> {
    let mut regs: Vec<Reg> = Vec::new();
    for i in 0..width {
        let shift = init >> i;
        let mask = shift & 1;
        let is_one = mask == 1;
        if is_one {
            regs.push(Reg::new_fdse());
        } else {
            regs.push(Reg::new_fdre());
        }
    }
    regs
}
