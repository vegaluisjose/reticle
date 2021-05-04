use xpand::lut::Lut2;

pub fn main() {
    let mut lut = Lut2::default();
    lut.set_name("i0");
    println!("{}", lut.to_instance());
}
