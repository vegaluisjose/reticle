use translate::driver::Driver;
use translate::errors::Error;

fn main() -> Result<(), Error> {
    let driver = Driver::default();
    driver.run()?;
    Ok(())
}
