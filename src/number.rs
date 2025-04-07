use std::{num::Wrapping, ops::Add};

// Number defines all the numbers types there are within the query language.
// Numbers can be added together, and the result will be a type which can fit the
// underlying value. For instance:
//
// 12.0 + 5 = 17.0
//
// Since one of the values is a float, the result will be a float. However lets look
// at another example:
//
// 4 + 9 = 13
//
// Both numbers are unsigned integers, so the returned number will also be an unsigned
// integer.
pub enum Number {
    Float(f64),
    Integer(i64),
    UInteger(u64),
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => Number::Float(lhs + rhs),
            (Self::Float(lhs), Self::Integer(rhs)) => Number::Float(lhs + rhs as f64),
            (Self::Float(lhs), Self::UInteger(rhs)) => Number::Float(lhs + rhs as f64),
            (Self::Integer(lhs), Self::Float(rhs)) => Number::Float(lhs as f64 + rhs),
            (Self::Integer(lhs), Self::Integer(rhs)) => {
                Number::Integer((Wrapping(lhs) + Wrapping(rhs)).0)
            }
            (Self::Integer(lhs), Self::UInteger(rhs)) => {
                if lhs < 0 {
                    // This can panic if the u64 is too big to fit into a i64
                    // fix that
                    Number::Integer((Wrapping(lhs) + Wrapping(rhs as i64)).0)
                } else {
                    Number::UInteger((Wrapping(lhs as u64) + Wrapping(rhs)).0)
                }
            }
            (Self::UInteger(lhs), Self::Float(rhs)) => Number::Float(lhs as f64 + rhs),
            (Self::UInteger(lhs), Self::Integer(rhs)) => {
                if rhs < 0 {
                    // This can panic if the u64 is too big to fit into a i64
                    // fix that
                    Number::Integer((Wrapping(lhs as i64) + Wrapping(rhs)).0)
                } else {
                    Number::UInteger((Wrapping(lhs) + Wrapping(rhs as u64)).0)
                }
            }
            (Self::UInteger(lhs), Self::UInteger(rhs)) => {
                Number::UInteger((Wrapping(lhs) + Wrapping(rhs)).0)
            }
        }
    }
}
