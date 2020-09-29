use crate::asm::ast as asm;
use crate::backend::arch::ultrascale::assembler::Assembler;
use crate::lang::ast as lang;
use std::collections::HashSet;
use vast::v05::ast as verilog;

pub use verilog::*;

impl From<asm::Prog> for verilog::Module {
    fn from(prog: asm::Prog) -> Self {
        let mut assembler = Assembler::default();
        assembler.emit(prog)
    }
}

impl From<lang::Expr> for Vec<verilog::Id> {
    fn from(expr: lang::Expr) -> Self {
        let mut ids: Vec<verilog::Id> = Vec::new();
        if expr.ty().is_vector() {
            for i in 0..expr.ty().length() {
                ids.push(format!("{}_{}", expr.id(), i));
            }
        } else {
            ids.push(expr.id());
        }
        ids
    }
}

impl From<lang::Port> for Vec<verilog::Port> {
    fn from(port: lang::Port) -> Self {
        let mut ports: Vec<verilog::Port> = Vec::new();
        let width = port.ty().width();
        let names: Vec<verilog::Id> = port.expr().clone().into();
        for n in names {
            let port = if port.is_input() {
                verilog::Port::new_input(&n, width)
            } else {
                verilog::Port::new_output(&n, width)
            };
            ports.push(port);
        }
        ports
    }
}

impl From<lang::Sig> for Vec<verilog::Port> {
    fn from(sig: lang::Sig) -> Self {
        let mut ports: Vec<verilog::Port> = Vec::new();
        for p in sig.inputs() {
            let v: Vec<verilog::Port> = p.clone().into();
            ports.extend(v);
        }
        for p in sig.outputs() {
            let v: Vec<verilog::Port> = p.clone().into();
            ports.extend(v);
        }
        ports
    }
}

impl From<lang::Instr> for Vec<verilog::Decl> {
    fn from(instr: lang::Instr) -> Self {
        let mut decls: Vec<verilog::Decl> = Vec::new();
        let expr = instr.dst();
        let width = expr.ty().width();
        let names: Vec<verilog::Id> = expr.clone().into();
        for n in names {
            let decl = if instr.is_reg() {
                verilog::Decl::new_reg(&n, width)
            } else {
                verilog::Decl::new_wire(&n, width)
            };
            decls.push(decl);
        }
        decls
    }
}

impl From<lang::InstrStd> for Vec<verilog::Stmt> {
    fn from(instr: lang::InstrStd) -> Self {
        match instr.op() {
            lang::StdOp::Identity => {
                let inp = verilog::Expr::new_ref(&instr.indexed_param(0).id());
                let out = verilog::Expr::new_ref(&instr.dst_id());
                let assign = verilog::Parallel::Assign(out, inp);
                vec![verilog::Stmt::from(assign)]
            }
            lang::StdOp::Const => {
                if instr.is_vector() {
                    unimplemented!()
                } else {
                    let attr = &instr.indexed_attr(0).value();
                    let width = instr.dst_ty().width();
                    let value = verilog::Expr::new_ulit_dec(width as u32, &attr.to_string());
                    let out = verilog::Expr::new_ref(&instr.dst_id());
                    let assign = verilog::Parallel::Assign(out, value);
                    vec![verilog::Stmt::from(assign)]
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl From<lang::InstrPrim> for Vec<verilog::Stmt> {
    fn from(instr: lang::InstrPrim) -> Self {
        let mut stmts: Vec<verilog::Stmt> = Vec::new();
        let lhs: Vec<verilog::Id> = instr.indexed_param(0).clone().into();
        let rhs: Vec<verilog::Id> = instr.indexed_param(1).clone().into();
        let res: Vec<verilog::Id> = instr.dst().clone().into();
        match instr.op() {
            lang::PrimOp::Add => {
                for ((a, b), y) in lhs.iter().zip(rhs.iter()).zip(res.iter()) {
                    let ar = Expr::new_signed_ref(a);
                    let br = Expr::new_signed_ref(b);
                    let yr = Expr::new_ref(y);
                    let add = verilog::Expr::new_add(ar, br);
                    let assign = verilog::Parallel::Assign(yr, add);
                    stmts.push(verilog::Stmt::from(assign));
                }
            }
            lang::PrimOp::Mul => {
                for ((a, b), y) in lhs.iter().zip(rhs.iter()).zip(res.iter()) {
                    let ar = Expr::new_signed_ref(a);
                    let br = Expr::new_signed_ref(b);
                    let yr = Expr::new_ref(y);
                    let add = verilog::Expr::new_mul(ar, br);
                    let assign = verilog::Parallel::Assign(yr, add);
                    stmts.push(verilog::Stmt::from(assign));
                }
            }
            lang::PrimOp::Reg => {
                let en = Expr::new_ref(&instr.indexed_param(1).id());
                for (i, (a, y)) in lhs.iter().zip(res.iter()).enumerate() {
                    let ar = Expr::new_ref(a);
                    let yr = Expr::new_ref(y);
                    let event = verilog::Sequential::new_posedge("clock");
                    let mut always = verilog::ParallelAlways::new(event);
                    let reset = verilog::Expr::new_ref("reset");
                    let val = if instr.attrs().len() == 1 {
                        verilog::Expr::new_int(instr.indexed_attr(0).value() as i32)
                    } else {
                        verilog::Expr::new_int(instr.indexed_attr(i).value() as i32)
                    };
                    let s0 = verilog::Sequential::new_nonblk_assign(yr.clone(), val);
                    let s1 = verilog::Sequential::new_nonblk_assign(yr, ar);
                    let mut i0 = verilog::SequentialIfElse::new(reset);
                    let mut i1 = verilog::SequentialIfElse::new(en.clone());
                    i0.add_seq(s0);
                    i1.add_seq(s1);
                    i0.set_else(i1.into());
                    always.add_seq(i0.into());
                    stmts.push(verilog::Stmt::from(always));
                }
            }
            _ => unimplemented!(),
        }
        stmts
    }
}

impl From<lang::Instr> for Vec<verilog::Stmt> {
    fn from(instr: lang::Instr) -> Self {
        if instr.is_std() {
            instr.std().clone().into()
        } else {
            instr.prim().clone().into()
        }
    }
}

impl From<lang::Prog> for verilog::Module {
    fn from(prog: lang::Prog) -> Self {
        let def = prog.indexed_def(0);
        let mut ports: Vec<verilog::Port> = Vec::new();
        ports.push(verilog::Port::new_input("clock", 1));
        ports.push(verilog::Port::new_input("reset", 1));
        let def_ports: Vec<verilog::Port> = def.signature().clone().into();
        ports.extend(def_ports);
        let outputs: HashSet<lang::Id> = def.signature().outputs().iter().map(|x| x.id()).collect();
        let mut module = Module::new(&def.id());
        for p in ports {
            module.add_port(p);
        }
        // decls
        for instr in def.body() {
            if !outputs.contains(&instr.dst().id()) {
                let decl: Vec<verilog::Decl> = instr.clone().into();
                for d in decl {
                    module.add_stmt(verilog::Stmt::from(d));
                }
            }
        }
        // exprs
        for instr in def.body() {
            let stmts: Vec<verilog::Stmt> = instr.clone().into();
            for s in stmts {
                module.add_stmt(s);
            }
        }
        module
    }
}
