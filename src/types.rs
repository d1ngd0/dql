use crate::Error;
use std::cmp::Ordering;
use std::ops::{Div, Mul, Rem, Sub};
use std::{collections::HashMap, fmt::Debug, fmt::Display};
use std::{num::Wrapping, ops::Add};

// Any defines all the data types that the query language can support
#[derive(Debug)]
pub enum Any<'a> {
    Null,
    Str(Str<'a>),
    Number(Number),
    Bool(bool),
    List(Vec<Any<'a>>),
    Map(HashMap<String, Any<'a>>),
}

macro_rules! impl_any_from {
    ($type:ty, $variant:ident) => {
        impl<'a> From<$type> for Any<'a> {
            fn from(value: $type) -> Self {
                Any::$variant(value)
            }
        }
    };
}
impl_any_from!(Number, Number);
impl_any_from!(Str<'a>, Str);
impl_any_from!(bool, Bool);
impl_any_from!(Vec<Any<'a>>, List);
impl_any_from!(HashMap<String, Any<'a>>, Map);

impl Default for Any<'_> {
    fn default() -> Self {
        Any::Null
    }
}

// Display makes it possible to show the string value
impl Display for Any<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: This needs to be syntactically correct
        Debug::fmt(self, f)
    }
}

impl PartialEq for Any<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Str(lhs), Self::Str(rhs)) => lhs.eq(rhs),
            (Self::Number(lhs), Self::Number(rhs)) => lhs.eq(rhs),
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs.eq(rhs),
            (Self::List(lhs), Self::List(rhs)) => lhs.eq(rhs),
            (Self::Map(lhs), Self::Map(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

impl Eq for Any<'_> {}

impl PartialOrd for Any<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Null, _) => Some(Ordering::Less),
            (_, Self::Null) => Some(Ordering::Greater),
            (Self::Str(lhs), Self::Str(rhs)) => lhs.partial_cmp(rhs),
            (Self::Number(lhs), Self::Number(rhs)) => lhs.partial_cmp(rhs),
            (Self::Bool(lhs), Self::Bool(rhs)) => Some(lhs.cmp(rhs)),
            (Self::List(lhs), Self::List(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
    }
}

// Str is a union that allows an owned string or a string reference
// This allows the underlying container to decide how to return the
// underlying data, potentially saving heap allocations when &str can
// be used.
#[derive(Debug)]
pub enum Str<'a> {
    String(String),
    Str(&'a str),
}

impl From<Str<'_>> for String {
    fn from(value: Str) -> Self {
        match value {
            Str::String(v) => v,
            Str::Str(v) => String::from(v),
        }
    }
}

impl<'a> From<&'a Str<'a>> for &'a str {
    fn from(value: &'a Str<'a>) -> Self {
        match value {
            Str::String(v) => v.as_str(),
            Str::Str(v) => v,
        }
    }
}

macro_rules! impl_string_from {
    ($type:ty, $variant:ident) => {
        impl<'a> From<$type> for Str<'a> {
            fn from(value: $type) -> Self {
                Str::$variant(value)
            }
        }

        impl<'a> From<$type> for Any<'a> {
            fn from(value: $type) -> Self {
                Any::Str(Str::$variant(value))
            }
        }
    };
}

impl_string_from!(String, String);
impl_string_from!(&'a str, Str);
impl_string_from!(&'a String, Str);

// TryFrom makes it possible to turn an Any into a Str if the underlying
// type matches, if the value can not be turned into a str it will return
// Error::InvalidType
impl<'a> TryFrom<Any<'a>> for Str<'a> {
    type Error = Error;

    fn try_from(value: Any<'a>) -> Result<Self, Self::Error> {
        match value {
            Any::Str(v) => Ok(v),
            _ => Err(Error::InvalidType),
        }
    }
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
        // TODO: This needs to be syntactically correct
        Debug::fmt(self, f)
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
#[derive(Debug)]
pub enum Number {
    Float(f64),
    Integer(i64),
    UInteger(u64),
}

// TryFrom makes it possible to turn an Any into a Number if the underlying
// type matches, if the value can not be turned into a Number it will return
// Error::InvalidType
impl TryFrom<Any<'_>> for Number {
    type Error = Error;

    fn try_from(value: Any<'_>) -> Result<Self, Self::Error> {
        match value {
            Any::Number(v) => Ok(v),
            _ => Err(Error::InvalidType),
        }
    }
}

macro_rules! impl_number_op {
    ($name:ident, $fn:ident, $op:tt) => {
        impl $name for Number {
            type Output = Number;

            fn $fn(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Self::Float(lhs), Self::Float(rhs)) => Number::Float(lhs $op rhs),
                    (Self::Float(lhs), Self::Integer(rhs)) => Number::Float(lhs $op rhs as f64),
                    (Self::Float(lhs), Self::UInteger(rhs)) => Number::Float(lhs $op rhs as f64),
                    (Self::Integer(lhs), Self::Float(rhs)) => Number::Float(lhs as f64 $op rhs),
                    (Self::Integer(lhs), Self::Integer(rhs)) => Number::Integer((Wrapping(lhs) $op Wrapping(rhs)).0),
                    (Self::Integer(lhs), Self::UInteger(rhs)) => Number::Integer((Wrapping(lhs) $op Wrapping(rhs as i64)).0),
                    (Self::UInteger(lhs), Self::Float(rhs)) => Number::Float(lhs as f64 $op rhs),
                    (Self::UInteger(lhs), Self::Integer(rhs)) => Number::Integer((Wrapping(lhs as i64) $op Wrapping(rhs)).0),
                    (Self::UInteger(lhs), Self::UInteger(rhs)) => Number::UInteger((Wrapping(lhs) $op Wrapping(rhs)).0),
                }
            }
        }
    }
}

impl_number_op!(Add, add, +);
impl_number_op!(Sub, sub, -);
impl_number_op!(Mul, mul, *);
impl_number_op!(Div, div, /);
impl_number_op!(Rem, rem, %);

macro_rules! impl_number_from {
    ($type:ty, $variant:ident, $cast:ident) => {
        impl From<Number> for $type {
            fn from(orig: Number) -> Self {
                match orig {
                    Number::Float(num) => num as $type,
                    Number::Integer(num) => num as $type,
                    Number::UInteger(num) => num as $type,
                }
            }
        }

        impl From<$type> for Number {
            fn from(orig: $type) -> Self {
                Number::$variant(orig as $cast)
            }
        }

        impl From<$type> for Any<'_> {
            fn from(orig: $type) -> Self {
                Any::Number(Number::$variant(orig as $cast))
            }
        }
    };
}
impl_number_from!(u8, UInteger, u64);
impl_number_from!(u16, UInteger, u64);
impl_number_from!(u32, UInteger, u64);
impl_number_from!(u64, UInteger, u64);
impl_number_from!(u128, UInteger, u64);
impl_number_from!(i8, Integer, i64);
impl_number_from!(i16, Integer, i64);
impl_number_from!(i32, Integer, i64);
impl_number_from!(i64, Integer, i64);
impl_number_from!(i128, Integer, i64);
impl_number_from!(f32, Float, f64);
impl_number_from!(f64, Float, f64);

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

// Support <, > etc and enable sorting
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Float(lhs), Self::Float(rhs)) => lhs.partial_cmp(rhs),
            (Self::Float(lhs), Self::Integer(rhs)) => {
                let rhs = *rhs as f64;
                lhs.partial_cmp(&rhs)
            }
            (Self::Float(lhs), Self::UInteger(rhs)) => {
                let rhs = *rhs as f64;
                lhs.partial_cmp(&rhs)
            }
            (Self::Integer(lhs), Self::Float(rhs)) => {
                let lhs = *lhs as f64;
                lhs.partial_cmp(rhs)
            }
            (Self::Integer(lhs), Self::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Self::Integer(lhs), Self::UInteger(rhs)) => {
                if *lhs < 0 {
                    Some(Ordering::Less)
                } else {
                    let lhs = *lhs as u64;
                    lhs.partial_cmp(rhs)
                }
            }
            (Self::UInteger(lhs), Self::Float(rhs)) => {
                let lhs = *lhs as f64;
                lhs.partial_cmp(rhs)
            }
            (Self::UInteger(lhs), Self::Integer(rhs)) => {
                if *rhs < 0 {
                    Some(Ordering::Greater)
                } else {
                    let rhs = *rhs as u64;
                    lhs.partial_cmp(&rhs)
                }
            }
            (Self::UInteger(lhs), Self::UInteger(rhs)) => lhs.partial_cmp(rhs),
        }
    }
}

// Display makes it possible to show the string value
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: This needs to be syntactically correct
        Debug::fmt(self, f)
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
    fn test_number_add() {
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

    #[test]
    fn test_number_partial_order() {
        // float
        assert!(Number::Float(20.0) > Number::Float(15.0));
        assert!(Number::Float(20.0) >= Number::Float(15.0));
        assert!(Number::Float(20.0) >= Number::Float(15.0));
        assert!(Number::Float(15.0) < Number::Float(20.0));
        assert!(Number::Float(15.0) <= Number::Float(20.0));
        assert!(Number::Float(15.0) <= Number::Float(20.0));
        assert!(Number::Float(20.0) > Number::Integer(15));
        assert!(Number::Float(20.0) >= Number::Integer(15));
        assert!(Number::Float(20.0) >= Number::Integer(15));
        assert!(Number::Float(15.0) < Number::Integer(20));
        assert!(Number::Float(15.0) <= Number::Integer(20));
        assert!(Number::Float(15.0) <= Number::Integer(20));
        assert!(Number::Float(20.0) > Number::UInteger(15));
        assert!(Number::Float(20.0) >= Number::UInteger(15));
        assert!(Number::Float(20.0) >= Number::UInteger(15));
        assert!(Number::Float(15.0) < Number::UInteger(20));
        assert!(Number::Float(15.0) <= Number::UInteger(20));
        assert!(Number::Float(15.0) <= Number::UInteger(20));

        // Integer
        assert!(Number::Integer(20) > Number::Float(15.0));
        assert!(Number::Integer(20) >= Number::Float(15.0));
        assert!(Number::Integer(20) >= Number::Float(15.0));
        assert!(Number::Integer(15) < Number::Float(20.0));
        assert!(Number::Integer(15) <= Number::Float(20.0));
        assert!(Number::Integer(15) <= Number::Float(20.0));
        assert!(Number::Integer(20) > Number::Integer(15));
        assert!(Number::Integer(20) >= Number::Integer(15));
        assert!(Number::Integer(20) >= Number::Integer(15));
        assert!(Number::Integer(15) < Number::Integer(20));
        assert!(Number::Integer(15) <= Number::Integer(20));
        assert!(Number::Integer(15) <= Number::Integer(20));
        assert!(Number::Integer(20) > Number::UInteger(15));
        assert!(Number::Integer(20) >= Number::UInteger(15));
        assert!(Number::Integer(20) >= Number::UInteger(15));
        assert!(Number::Integer(15) < Number::UInteger(20));
        assert!(Number::Integer(15) <= Number::UInteger(20));
        assert!(Number::Integer(15) <= Number::UInteger(20));

        // UInteger
        assert!(Number::UInteger(20) > Number::Float(15.0));
        assert!(Number::UInteger(20) >= Number::Float(15.0));
        assert!(Number::UInteger(20) >= Number::Float(15.0));
        assert!(Number::UInteger(15) < Number::Float(20.0));
        assert!(Number::UInteger(15) <= Number::Float(20.0));
        assert!(Number::UInteger(15) <= Number::Float(20.0));
        assert!(Number::UInteger(20) > Number::Integer(15));
        assert!(Number::UInteger(20) >= Number::Integer(15));
        assert!(Number::UInteger(20) >= Number::Integer(15));
        assert!(Number::UInteger(15) < Number::Integer(20));
        assert!(Number::UInteger(15) <= Number::Integer(20));
        assert!(Number::UInteger(15) <= Number::Integer(20));
        assert!(Number::UInteger(20) > Number::UInteger(15));
        assert!(Number::UInteger(20) >= Number::UInteger(15));
        assert!(Number::UInteger(20) >= Number::UInteger(15));
        assert!(Number::UInteger(15) < Number::UInteger(20));
        assert!(Number::UInteger(15) <= Number::UInteger(20));
        assert!(Number::UInteger(15) <= Number::UInteger(20));
    }

    #[test]
    fn test_any() {
        assert_eq!(Any::Bool(true), Any::Bool(true));
        assert_ne!(Any::Number(Number::Float(23.5)), Any::Bool(true));
        assert_ne!(Any::Str(Str::Str("hello")), Any::Null);
    }
}
