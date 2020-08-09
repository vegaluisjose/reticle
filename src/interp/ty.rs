use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Scalar { value: i64 },
    Vector { values: Vec<i64> },
}

pub type Map = HashMap<String, Value>;

impl Value {
    pub fn new_scalar(num: i64) -> Value {
        Value::Scalar { value: num }
    }

    pub fn new_vector() -> Value {
        Value::Vector { values: Vec::new() }
    }

    pub fn get_scalar(&self) -> i64 {
        match self {
            Value::Scalar { value } => *value,
            _ => panic!("Error: value is not a scalar"),
        }
    }

    pub fn get_vector(&self) -> &Vec<i64> {
        match self {
            Value::Vector { values } => values,
            _ => panic!("Error: value is not a vector"),
        }
    }

    pub fn push(&mut self, num: i64) {
        match self {
            Value::Vector { values } => values.push(num),
            _ => panic!("Error: value is not a vector"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Scalar { value } => write!(f, "{}", value),
            Value::Vector { values } => write!(f, "{:?}", values),
        }
    }
}

impl From<Vec<i64>> for Value {
    fn from(values: Vec<i64>) -> Self {
        Value::Vector { values }
    }
}
