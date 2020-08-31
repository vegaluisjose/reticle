use reticle::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};

fn main() {
    let mut dsp = Dsp::new_scalar(DspOp::Add);
    dsp.set_id("i0");
    println!("{}", dsp);
}
