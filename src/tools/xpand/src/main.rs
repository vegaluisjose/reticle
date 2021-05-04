use xpand::carry::Carry;

pub fn main() {
    let mut carry = Carry::default();
    carry.set_name("i0");
    println!("{}", carry.to_instance());
}
