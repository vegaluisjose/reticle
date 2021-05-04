use xpand::fdre::Fdre;

pub fn main() {
    let mut fdre = Fdre::default();
    fdre.set_name("i0");
    println!("{}", fdre.to_instance());
}
