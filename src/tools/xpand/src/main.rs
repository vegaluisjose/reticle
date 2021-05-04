use xpand::lut::Lut3;

pub fn main() {
    let mut lut = Lut3::default();
    lut.set_name("i0");
    println!("{}", lut.to_instance());
}
