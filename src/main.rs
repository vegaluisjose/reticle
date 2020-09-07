use reticle::driver::Driver;

fn main() {
    let driver = Driver::default();
    println!("{:?}", driver.opts());
}
