use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg())
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

impl Error {
    pub fn msg(&self) -> String {
        match self {
            Error::ParseError(n) => n.to_string(),
        }
    }
}
