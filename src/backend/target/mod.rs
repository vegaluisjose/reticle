mod descriptor;
pub mod from;
pub mod from_str;
pub mod helpers;
pub mod spec;

pub use descriptor::*;

pub trait Target {
    fn to_descriptor(&self) -> descriptor::Descriptor;
}
