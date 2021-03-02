use reticle::driver::optimize::OptimizeDriver;
use reticle::util::errors::Error;

fn main() -> Result<(), Error> {
    let driver = OptimizeDriver::default();
    driver.run()?;
    Ok(())
}
