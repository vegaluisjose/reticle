use crate::backend::asm::ast as asm;
use crate::backend::asm::verilog::ToVerilog;
use vast::v05::ast as verilog;

fn to_verilog_port(port: asm::Port) -> Vec<verilog::Port> {
    let mut ports: Vec<verilog::Port> = Vec::new();
    match port {
        asm::Port::Input { id, ty } => {
            if ty.is_vector() {
                for i in 0..ty.length() {
                    let name = format!("{}_{}", id, i);
                    let port = verilog::Port::new_input(&name, ty.width());
                    ports.push(port);
                }
            } else {
                let port = verilog::Port::new_input(&id, ty.width());
                ports.push(port);
            }
        }
        asm::Port::Output { id, ty } => {
            if ty.is_vector() {
                for i in 0..ty.length() {
                    let name = format!("{}_{}", id, i);
                    let port = verilog::Port::new_output(&name, ty.width());
                    ports.push(port);
                }
            } else {
                let port = verilog::Port::new_output(&id, ty.width());
                ports.push(port);
            }
        }
    }
    ports
}

// fn to_verilog_body(instr: asm::Instr) -> Vec<verilog::Stmt> {
//     use crate::backend::arch::ultrascale::isa;
//     if instr.is_prim() {
//         match instr.prim_op().as_ref() {
//             "lut_and_bool_bool_bool" => isa::lut_and_bool_bool_bool(instr),
//             _ => vec![],
//         }
//     } else {
//         vec![]
//     }
// }

// impl From<asm::Prog> for verilog::Module {
//     fn from(prog: asm::Prog) -> Self {
//         let mut ports: Vec<verilog::Port> = Vec::new();
//         for input in prog.inputs().iter() {
//             ports.extend(to_verilog_port(input.clone()));
//         }
//         for output in prog.outputs().iter() {
//             ports.extend(to_verilog_port(output.clone()));
//         }
//         let mut body: Vec<verilog::Stmt> = Vec::new();
//         for instr in prog.body().iter() {
//             body.extend(to_verilog_body(instr.clone()));
//         }
//         let mut module = verilog::Module::new(&prog.id());
//         for port in ports.iter() {
//             module.add_port(port.clone());
//         }
//         for stmt in body.iter() {
//             module.add_stmt(stmt.clone());
//         }
//         module
//     }
// }

// impl ToVerilog for asm::Prog {
//     fn to_verilog(&self) -> verilog::Module {
//         verilog::Module::from(self.clone())
//     }
// }

#[derive(Clone, Debug)]
pub struct Assembler {
    pub prog: asm::Prog,
}

impl Assembler {
    pub fn new(prog: asm::Prog) -> Assembler {
        Assembler { prog }
    }
    pub fn prog(&self) -> &asm::Prog {
        &self.prog
    }
}

impl ToVerilog for Assembler {
    fn to_verilog(&self) -> verilog::Module {
        let mut ports: Vec<verilog::Port> = Vec::new();
        for input in self.prog().inputs().iter() {
            ports.extend(to_verilog_port(input.clone()));
        }
        for output in self.prog().outputs().iter() {
            ports.extend(to_verilog_port(output.clone()));
        }
        let mut module = verilog::Module::new(&self.prog().id());
        for port in ports.iter() {
            module.add_port(port.clone());
        }
        module
    }
}
