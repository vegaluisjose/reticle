use optimizer::driver::Driver;
use optimizer::errors::Error;

fn main() -> Result<(), Error> {
    let driver = Driver::default();
    driver.run()?;
    Ok(())
}
