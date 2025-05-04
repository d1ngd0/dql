use crate::{Container, Error};
use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Div, Mul, Rem, Sub};
use std::{collections::HashMap, fmt::Debug, fmt::Display};
use std::{num::Wrapping, ops::Add};

// Any defines all the data types that the query language can support
#[derive(Debug, Clone)]
pub enum Any<'a> {
    Null,
    Str(Str<'a>),
    Bytes(Bytes<'a>),
    Number(Number),
    Bool(bool),
    List(Vec<Any<'a>>),
    Map(HashMap<Str<'a>, Any<'a>>),
}

impl<'a> Any<'a> {
    pub fn as_str(&'a self) -> Result<&'a str, Error> {
        match self {
            Any::Str(v) => Ok(v.as_str()),
            _ => Err(Error::InvalidType),
        }
    }

    pub fn as_slice(&'a self) -> Result<&'a [u8], Error> {
        match self {
            Any::Bytes(v) => Ok(v.as_ref()),
            _ => Err(Error::InvalidType),
        }
    }
}

impl Container for Any<'_> {}

macro_rules! impl_any_from {
    ($type:ty, $variant:ident) => {
        impl<'a> From<$type> for Any<'a> {
            fn from(value: $type) -> Self {
                Any::$variant(value.into())
            }
        }
    };
}

macro_rules! impl_any_try_from {
    ($type:ty, $variant:ident) => {
        impl<'a> TryFrom<Any<'a>> for $type {
            type Error = Error;

            fn try_from(value: Any<'a>) -> Result<Self, Self::Error> {
                match value {
                    Any::$variant(v) => Ok(v.into()),
                    _ => Err(Error::InvalidType),
                }
            }
        }
    };
}
impl_any_from!(Number, Number);
impl_any_try_from!(Number, Number);
impl_any_from!(usize, Number);
impl_any_try_from!(usize, Number);
impl_any_from!(u64, Number);
impl_any_try_from!(u64, Number);
impl_any_from!(u32, Number);
impl_any_try_from!(u32, Number);
impl_any_from!(u16, Number);
impl_any_try_from!(u16, Number);
impl_any_from!(u8, Number);
impl_any_try_from!(u8, Number);
impl_any_from!(isize, Number);
impl_any_try_from!(isize, Number);
impl_any_from!(i64, Number);
impl_any_try_from!(i64, Number);
impl_any_from!(i32, Number);
impl_any_try_from!(i32, Number);
impl_any_from!(i16, Number);
impl_any_try_from!(i16, Number);
impl_any_from!(i8, Number);
impl_any_try_from!(i8, Number);
impl_any_from!(f64, Number);
impl_any_try_from!(f64, Number);
impl_any_from!(f32, Number);
impl_any_try_from!(f32, Number);
impl_any_from!(Str<'a>, Str);
impl_any_try_from!(Str<'a>, Str);
impl_any_from!(String, Str);
impl_any_try_from!(String, Str);
impl_any_from!(&'a String, Str);
impl_any_from!(&'a str, Str);
impl_any_from!(Bytes<'a>, Bytes);
impl_any_try_from!(Bytes<'a>, Bytes);
impl_any_from!(Vec<u8>, Bytes);
impl_any_try_from!(Vec<u8>, Bytes);
impl_any_from!(&'a Vec<u8>, Bytes);
impl_any_from!(&'a [u8], Bytes);
impl_any_from!(bool, Bool);
impl_any_try_from!(bool, Bool);
impl_any_from!(Vec<Any<'a>>, List);
impl_any_try_from!(Vec<Any<'a>>, List);
impl_any_from!(HashMap<Str<'a>, Any<'a>>, Map);
impl_any_try_from!(HashMap<Str<'a>, Any<'a>>, Map);

impl<'a> TryFrom<&'a Any<'a>> for &'a str {
    type Error = Error;

    fn try_from(value: &'a Any<'a>) -> Result<Self, Self::Error> {
        match value {
            Any::Str(v) => Ok(v.into()),
            _ => Err(Error::InvalidType),
        }
    }
}

impl<'a> TryFrom<&'a Any<'a>> for &'a [u8] {
    type Error = Error;

    fn try_from(value: &'a Any<'a>) -> Result<Self, Self::Error> {
        match value {
            Any::Bytes(v) => Ok(v.into()),
            _ => Err(Error::InvalidType),
        }
    }
}

impl<'a, const SIZE: usize> From<[Any<'a>; SIZE]> for Any<'a> {
    fn from(value: [Any<'a>; SIZE]) -> Self {
        Any::List(value.into())
    }
}

impl<'a, const SIZE: usize> From<[(Str<'a>, Any<'a>); SIZE]> for Any<'a> {
    fn from(value: [(Str<'a>, Any<'a>); SIZE]) -> Self {
        Any::Map(value.into())
    }
}

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
            (Self::Bytes(lhs), Self::Bytes(rhs)) => lhs.eq(rhs),
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
            (Self::Bytes(lhs), Self::Bytes(rhs)) => lhs.partial_cmp(rhs),
            (Self::Number(lhs), Self::Number(rhs)) => lhs.partial_cmp(rhs),
            (Self::Bool(lhs), Self::Bool(rhs)) => Some(lhs.cmp(rhs)),
            (Self::List(lhs), Self::List(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
    }
}

impl Hash for Any<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Null => state.write(&[0x0 as u8]),
            Self::Str(str) => str.hash(state),
            Self::Bytes(b) => b.hash(state),
            Self::Number(num) => num.hash(state),
            Self::Bool(b) => b.hash(state),
            Self::List(list) => list.hash(state),
            Self::Map(map) => {
                let mut hash: u64 = 0;

                for (k, v) in map {
                    let mut h = DefaultHasher::new();
                    h.write(k.as_str().as_bytes());
                    v.hash(&mut h);
                    hash ^= h.finish();
                }

                state.write_u64(hash);
            }
        }
    }
}

// Str is a union that allows an owned string or a string reference
// This allows the underlying container to decide how to return the
// underlying data, potentially saving heap allocations when &str can
// be used.
#[derive(Debug, Clone)]
pub enum Str<'a> {
    String(String),
    Str(&'a str),
}

impl<'a> From<&'a Str<'a>> for &'a str {
    fn from(value: &'a Str<'a>) -> Self {
        match value {
            Str::String(v) => v.as_str(),
            Str::Str(v) => v,
        }
    }
}

impl<'a> From<Str<'a>> for String {
    fn from(value: Str<'a>) -> Self {
        match value {
            Str::Str(v) => v.into(),
            Str::String(v) => v,
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
    };
}

impl_string_from!(String, String);
impl_string_from!(&'a str, Str);
impl_string_from!(&'a String, Str);

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

impl Hash for Str<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_str().as_bytes());
    }
}

// Bytes is a union that allows an owned string or a string reference
// This allows the underlying container to decide how to return the
// underlying data, potentially saving heap allocations when &str can
// be used.
#[derive(Debug, Clone)]
pub enum Bytes<'a> {
    Bytes(Vec<u8>),
    Ref(&'a [u8]),
}

impl From<Bytes<'_>> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        match value {
            Bytes::Bytes(v) => v,
            Bytes::Ref(v) => Vec::from(v),
        }
    }
}

impl<'a> From<&'a Bytes<'a>> for &'a [u8] {
    fn from(value: &'a Bytes<'a>) -> Self {
        match value {
            Bytes::Bytes(v) => v.as_ref(),
            Bytes::Ref(v) => v,
        }
    }
}

macro_rules! impl_string_from {
    ($type:ty, $variant:ident) => {
        impl<'a> From<$type> for Bytes<'a> {
            fn from(value: $type) -> Self {
                Bytes::$variant(value)
            }
        }
    };
}

impl_string_from!(Vec<u8>, Bytes);
impl_string_from!(&'a [u8], Ref);
impl_string_from!(&'a Vec<u8>, Ref);

impl<'a> Bytes<'a> {
    // as_str returns a reference to the underlying string
    pub fn as_ref(&self) -> &[u8] {
        match self {
            Self::Bytes(v) => v.as_ref(),
            Self::Ref(v) => v,
        }
    }

    // as string consumes the Bytes and returns a Vec, in the case
    // of an underlying &str it will copy the data and cause a heap
    // allocation.
    pub fn as_vec(self) -> Vec<u8> {
        match self {
            Self::Bytes(v) => v,
            Self::Ref(v) => Vec::from(v),
        }
    }
}

// PartialEq makes it possible to compare two strings
impl PartialEq for Bytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for Bytes<'_> {}

// PartialOrd implements >, <, >= and <=
impl PartialOrd for Bytes<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_ref().cmp(other.as_ref()))
    }
}

// Display makes it possible to show the string value
impl Display for Bytes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: This needs to be syntactically correct
        Debug::fmt(self, f)
    }
}

impl Hash for Bytes<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_ref());
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
#[derive(Debug, Copy, Clone)]
pub enum Number {
    Float(f64),
    Integer(i64),
    UInteger(u64),
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
    };
}
impl_number_from!(u8, UInteger, u64);
impl_number_from!(u16, UInteger, u64);
impl_number_from!(u32, UInteger, u64);
impl_number_from!(u64, UInteger, u64);
impl_number_from!(usize, UInteger, u64);
impl_number_from!(u128, UInteger, u64);
impl_number_from!(i8, Integer, i64);
impl_number_from!(i16, Integer, i64);
impl_number_from!(i32, Integer, i64);
impl_number_from!(i64, Integer, i64);
impl_number_from!(isize, Integer, i64);
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

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Float(f) => state.write_u64(f.to_bits()),
            Self::Integer(i) => state.write_i64(*i),
            Self::UInteger(u) => state.write_u64(*u),
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
