use crate::port::Output;
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub struct Vcc {
    pub name: String,
    pub prim: String,
    pub output: Output,
}

impl Default for Vcc {
    fn default() -> Self {
        let vcc = "vcc";
        let name = format!("_{}", vcc);
        Vcc {
            name,
            prim: "VCC".to_string(),
            output: Output::vcc(vcc),
        }
    }
}

impl Vcc {
    pub fn name(&self) -> String {
        self.name.to_string()
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
