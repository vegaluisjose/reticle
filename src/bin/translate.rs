use reticle::driver::translate::TranslateDriver;
use reticle::util::errors::Error;

fn main() -> Result<(), Error> {
    let driver = TranslateDriver::default();
    driver.run()?;
    Ok(())
}
