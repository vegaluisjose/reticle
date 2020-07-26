pub mod descriptor;
pub mod spec;

pub trait Target {
    fn to_descriptor(&self) -> descriptor::Descriptor;
}
