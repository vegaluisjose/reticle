use crate::util::errors::Error;
use crate::xl::ast::*;
use std::convert::TryFrom;
// use std::convert::TryInto;

impl TryFrom<OptVal> for u64 {
    type Error = Error;
    fn try_from(val: OptVal) -> Result<Self, Self::Error> {
        match val {
            OptVal::UInt(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}
