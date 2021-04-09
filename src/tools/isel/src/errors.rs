use ir::errors::Error as IRError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IR(IRError),
    Tree(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IR(msg) => write!(f, "{}", msg),
            Error::Tree(msg) => write!(f, "[Error][Tree] {}", msg),
        }
    }
}

impl Error {
    pub fn new_tree_error(msg: &str) -> Self {
        Error::Tree(msg.to_string())
    }
}

impl From<IRError> for Error {
    fn from(e: IRError) -> Self {
        Error::IR(e)
    }
}
