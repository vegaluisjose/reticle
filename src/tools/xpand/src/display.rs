use crate::fdre::Fdre;
use crate::instance::ToInstance;
use std::fmt;

macro_rules! display {
    ($ty:tt) => {
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_instance())
            }
        }
    };
}

display!(Fdre);
