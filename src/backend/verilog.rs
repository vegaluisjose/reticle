use crate::asm::ast as asm;
use crate::backend::arch::ultrascale::assembler::Assembler;
use crate::lang::ast as lang;
use vast::v05::ast as verilog;

pub use verilog::*;

impl From<asm::Prog> for verilog::Module {
    fn from(prog: asm::Prog) -> Self {
        let mut assembler = Assembler::default();
        assembler.emit(prog)
    }
}

impl From<lang::InstrStd> for verilog::Stmt {
    fn from(instr: lang::InstrStd) -> Self {
        match instr.op() {
            lang::StdOp::Identity => {
                let inp = verilog::Expr::new_ref(&instr.indexed_param(0).id());
                let out = verilog::Expr::new_ref(&instr.dst_id());
                let assign = verilog::Parallel::ParAssign(out, inp);
                verilog::Stmt::from(assign)
            }
            lang::StdOp::Const => {
                if instr.is_vector() {
                    unimplemented!()
                } else {
                    let attr = &instr.indexed_attr(0).value();
                    let width = instr.dst_ty().width();
                    let value = verilog::Expr::new_ulit_dec(width as u32, &attr.to_string());
                    let out = verilog::Expr::new_ref(&instr.dst_id());
                    let assign = verilog::Parallel::ParAssign(out, value);
                    verilog::Stmt::from(assign)
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl From<lang::Port> for Vec<verilog::Port> {
    fn from(port: lang::Port) -> Self {
        let mut ports: Vec<verilog::Port> = Vec::new();
        let width = port.ty().width();
        if port.ty().is_vector() {
            for i in 0..port.ty().length() {
                let name = format!("{}_{}", &port.id(), i);
                let vport = if port.is_input() {
                    verilog::Port::new_input(&name, width)
                } else {
                    verilog::Port::new_output(&name, width)
                };
                ports.push(vport);
            }
        } else {
            let sport = if port.is_input() {
                verilog::Port::new_input(&port.id(), width)
            } else {
                verilog::Port::new_output(&port.id(), width)
            };
            ports.push(sport);
        }
        ports
    }
}

impl From<lang::Sig> for Vec<verilog::Port> {
    fn from(sig: lang::Sig) -> Self {
        let mut ports: Vec<verilog::Port> = Vec::new();
        ports.push(verilog::Port::new_input("clock", 1));
        ports.push(verilog::Port::new_input("reset", 1));
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

impl From<lang::Prog> for verilog::Module {
    fn from(prog: lang::Prog) -> Self {
        let def = prog.indexed_def(0);
        let ports: Vec<verilog::Port> = def.signature().clone().into();
        let mut module = Module::new(&def.id());
        for p in ports {
            module.add_port(p);
        }
        module
    }
}
