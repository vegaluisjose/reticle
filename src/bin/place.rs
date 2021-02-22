use reticle::driver::place::PlaceDriver;
use reticle::util::errors::Error;

fn main() -> Result<(), Error> {
    let driver = PlaceDriver::default();
    driver.run()?;
    Ok(())
}
