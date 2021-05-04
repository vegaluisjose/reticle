use crate::port::Output;
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub struct Vcc {
    pub name: String,
    pub prim: String,
    pub wire: String,
    pub output: Output,
}

impl Default for Vcc {
    fn default() -> Self {
        let wire = "vcc";
        let name = format!("_{}", wire);
        Vcc {
            name,
            prim: "VCC".to_string(),
            wire: wire.to_string(),
            output: Output::vcc(wire),
        }
    }
}

impl Vcc {
    pub fn to_decl(&self) -> vl::Decl {
        vl::Decl::new_wire(&self.wire, 1)
    }
    pub fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        for (k, v) in self.output.connection.iter() {
            inst.connect(&k, v.clone());
        }
        inst
    }
    pub fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
}
