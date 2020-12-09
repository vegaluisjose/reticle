pub mod default;
pub mod display;
pub mod helpers;

#[derive(Clone, Debug)]
pub struct Analysis {
    prims: u32,
    stds: u32,
    holes: u32,
    luts: u32,
    dsps: u32,
    lums: u32,
    rams: u32,
}
