use std::cmp::Ordering;
use std::{collections::HashMap, fmt::Debug, fmt::Display};
use std::{num::Wrapping, ops::Add};

// Any defines all the data types that the query language can support
pub enum Any<'a> {
    Str(Str<'a>),
    Number(Number),
    Bool(bool),
    List(Vec<Any<'a>>),
    Map(HashMap<String, Any<'a>>),
}

// Str is a union that allows an owned string or a string reference
// This allows the underlying container to decide how to return the
// underlying data, potentially saving heap allocations when &str can
// be used.
pub enum Str<'a> {
    String(String),
    Str(&'a str),
}

impl<'a> Str<'a> {
    // as_str returns a reference to the underlying string
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(str) => str.as_str(),
            Self::Str(str) => str,
        }
    }

    // as string consumes the Str and returns a String, in the case
    // of an underlying &str it will copy the data and cause a heap
    // allocation.
    pub fn as_string(self) -> String {
        match self {
            Self::String(str) => str,
            Self::Str(str) => String::from(str),
        }
    }
}

// PartialEq makes it possible to compare two strings
impl PartialEq for Str<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Str<'_> {}

// PartialOrd implements >, <, >= and <=
impl PartialOrd for Str<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

// Display makes it possible to show the string value
impl Display for Str<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Display makes it possible to show the string value
impl Debug for Str<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

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

// Implement logic for +
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

// Add logic for implementing == and !=
impl PartialEq for Number {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Float(lhs), Self::Float(rhs)) => *lhs == *rhs,
            (Self::Float(lhs), Self::Integer(rhs)) => *lhs == *rhs as f64,
            (Self::Float(lhs), Self::UInteger(rhs)) => *lhs == *rhs as f64,
            (Self::Integer(lhs), Self::Float(rhs)) => *lhs as f64 == *rhs,
            (Self::Integer(lhs), Self::Integer(rhs)) => *lhs == *rhs,
            (Self::Integer(lhs), Self::UInteger(rhs)) => {
                if *lhs < 0 {
                    false
                } else {
                    *lhs as u64 == *rhs
                }
            }
            (Self::UInteger(lhs), Self::Float(rhs)) => *lhs as f64 == *rhs,
            (Self::UInteger(lhs), Self::Integer(rhs)) => {
                if *rhs < 0 {
                    false
                } else {
                    *lhs == *rhs as u64
                }
            }
            (Self::UInteger(lhs), Self::UInteger(rhs)) => *lhs == *rhs,
        }
    }
}

impl Eq for Number {}

// Display makes it possible to show the string value
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(n) => write!(f, "{}", n),
            Self::Integer(n) => write!(f, "{}", n),
            Self::UInteger(n) => write!(f, "{}", n),
        }
    }
}

// Display makes it possible to show the string value
impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(n) => write!(f, "{}", n),
            Self::Integer(n) => write!(f, "{}", n),
            Self::UInteger(n) => write!(f, "{}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_eq() {
        let hello = String::from("hello");
        assert_eq!(Str::String(String::from("hello")), Str::Str(hello.as_str()));
        assert_ne!(
            Str::String(String::from("goodbye")),
            Str::Str(hello.as_str())
        );
    }

    #[test]
    fn test_partial_order() {
        let hello = String::from("hello");
        let goodbye = String::from("goodbye");

        assert!(Str::Str(hello.as_str()) > Str::String(goodbye.clone()));
        assert!(Str::Str(hello.as_str()) >= Str::String(hello.clone()));
        assert!(Str::Str(hello.as_str()) <= Str::String(hello.clone()));
        assert!(Str::String(goodbye.clone()) < Str::Str(hello.as_str()));
    }

    #[test]
    fn test_number_eq() {
        assert_eq!(
            Number::Float(155.0) + Number::Integer(23),
            Number::Float(178.0)
        );
        assert_eq!(
            Number::Float(155.0) + Number::UInteger(23),
            Number::Float(178.0)
        );
        assert_eq!(
            Number::Float(155.0) + Number::Float(23.0),
            Number::Float(178.0)
        );
        assert_eq!(
            Number::Integer(155) + Number::Integer(23),
            Number::Float(178.0)
        );
        assert_eq!(
            Number::Integer(155) + Number::UInteger(23),
            Number::Float(178.0)
        );
        assert_eq!(
            Number::Integer(155) + Number::Float(23.0),
            Number::Float(178.0)
        );
    }
}
