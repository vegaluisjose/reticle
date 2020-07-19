pub mod ultrascale;
pub mod spec;
pub mod descriptor;

pub trait Target {
    fn to_descriptor(&self) -> descriptor::Descriptor;
}
