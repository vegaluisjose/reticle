pub mod default;
pub mod prim;
pub mod target;

pub struct Ultrascale {
    pub spec: String,
}

impl Ultrascale {
    fn spec(&self) -> String {
        self.spec.to_string()
    }
}
