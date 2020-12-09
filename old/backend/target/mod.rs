mod def;
pub mod from;
pub mod from_str;
pub mod helpers;
pub mod spec;

pub use def::*;

pub trait Target {
    fn to_descriptor(&self) -> Descriptor;
}
