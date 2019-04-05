use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Float(f64),
    Integer(i128),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Number::Float(num) => write!(f, "{}", num), 
            Number::Integer(num) => write!(f, "{}", num), 
        }
    }
}

pub type Name = String;