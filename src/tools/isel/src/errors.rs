#[derive(Debug)]
pub enum Error {
    Std(String),
}

impl Error {
    pub fn new_std_error(msg: &str) -> Self {
        Error::Std(msg.to_string())
    }
}
