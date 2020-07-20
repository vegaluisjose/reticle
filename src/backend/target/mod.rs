pub mod descriptor;
pub mod spec;
pub mod ultrascale;

pub trait Target {
    fn to_descriptor(&self) -> descriptor::Descriptor;
}
