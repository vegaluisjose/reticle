use crate::carry::Carry;
use crate::dsp::Dsp;
use crate::fdre::Fdre;
use crate::fdse::Fdse;
use crate::gnd::Gnd;
use crate::instance::ToInstance;
use crate::lut::{Lut1, Lut2, Lut3, Lut4, Lut5, Lut6};
use crate::vcc::Vcc;
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
display!(Fdse);
display!(Carry);
display!(Lut1);
display!(Lut2);
display!(Lut3);
display!(Lut4);
display!(Lut5);
display!(Lut6);
display!(Gnd);
display!(Vcc);
display!(Dsp);
