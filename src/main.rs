use reticle::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};

fn main() {
    let mut dsp = Dsp::new_scalar(DspOp::Add);
    dsp.set_id("i0");
    dsp.set_clock("clock");
    dsp.set_reset("reset");
    dsp.set_left("t0");
    dsp.set_right("t1");
    dsp.set_output("t2");
    println!("{}", dsp);
}
