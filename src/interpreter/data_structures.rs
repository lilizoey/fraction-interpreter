use std::{fmt, ops};

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

impl ops::Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Self::Output {
        use Number::*;

        match self {
            Integer(i) => 
                match other {
                    Integer(i2) => Integer(i + i2),
                    Float(f) => Float(i as f64 + f),
                },
            Float(f) => match other {
                Integer(i) => Float(i as f64 + f),
                Float(f2) => Float(f + f2), 
            }
            
        }
    }
}

pub type Name = String;