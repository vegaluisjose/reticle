use xpand::dsp::Dsp;

pub fn main() {
    let mut dsp = Dsp::default();
    dsp.set_name("i0");
    println!("{}", dsp.to_instance());
}
