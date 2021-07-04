use crate::prim::{Param, Port, PortSet};

impl From<&(&str, u32)> for Port {
    fn from(input: &(&str, u32)) -> Self {
        Port {
            name: input.0.into(),
            width: input.1,
        }
    }
}

impl From<(&str, u32)> for Port {
    fn from(input: (&str, u32)) -> Self {
        Port {
            name: input.0.into(),
            width: input.1,
        }
    }
}

impl From<&[(&str, u32)]> for PortSet {
    fn from(input: &[(&str, u32)]) -> Self {
        let mut set = PortSet::new();
        for t in input {
            set.insert(t.into());
        }
        set
    }
}

impl<T: Clone> From<&(&str, T)> for Param<T> {
    fn from(input: &(&str, T)) -> Self {
        Param {
            name: input.0.into(),
            value: input.1.clone(),
        }
    }
}

impl<T> From<(&str, T)> for Param<T> {
    fn from(input: (&str, T)) -> Self {
        Param {
            name: input.0.into(),
            value: input.1,
        }
    }
}
