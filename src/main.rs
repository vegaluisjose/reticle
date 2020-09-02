use reticle::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};

fn main() {
    let mut dsp = Dsp::new_scalar(DspOp::Add, 8);
    dsp.set_id("i0");
    dsp.set_clock("clock");
    dsp.set_reset("reset");
    println!("{}", dsp);
}
