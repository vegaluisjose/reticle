use xpand::fdre::Fdre;
use xpand::instance::ToInstance;

pub fn main() {
    let mut fdre = Fdre::default();
    fdre.set_name("i0");
    println!("{}", fdre.to_instance());
}
