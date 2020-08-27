pub mod assembler;
pub mod default;
pub mod isa;
pub mod lut;
pub mod reg;
pub mod target;

pub struct Ultrascale {
    pub spec: String,
}

impl Ultrascale {
    fn spec(&self) -> String {
        self.spec.to_string()
    }
}
