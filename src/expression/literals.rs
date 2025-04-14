use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::{Any, Container, Number, Parser, Result, Str, parser::STRING_WRAP};

use super::Expression;

// NullExpression is an expression that returns a null value.
#[derive(Debug)]
pub struct NullExpression {}

impl Default for NullExpression {
    fn default() -> Self {
        NullExpression {}
    }
}

impl<T: Container> Expression<T> for NullExpression {
    fn evaluate<'a: 'b, 'b>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::Null)
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(NullExpression {})
    }
}

impl Display for NullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

// StringExpression makes a literal string an expression.
#[derive(Debug)]
pub struct StringLiteral {
    value: String,
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        StringLiteral { value }
    }
}

impl<T: Container> Expression<T> for StringLiteral {
    fn evaluate<'a: 'b, 'b>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::from(&self.value))
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(StringLiteral {
            value: self.value.clone(),
        })
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", STRING_WRAP, self.value, STRING_WRAP)
    }
}

macro_rules! impl_deref_for_literal {
    ($type:ty, $target:ty) => {
        impl Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };
}
impl_deref_for_literal!(StringLiteral, String);

macro_rules! impl_string_literal_from {
    ($type:ty) => {
        impl<'a> From<$type> for StringLiteral {
            fn from(value: $type) -> Self {
                StringLiteral {
                    value: value.into(),
                }
            }
        }
    };
}
impl_string_literal_from!(String);
impl_string_literal_from!(&String);
impl_string_literal_from!(&str);
impl_string_literal_from!(Str<'a>);

// NumberExpression makes a literal string an expression.
#[derive(Debug)]
pub struct NumberLiteral {
    value: Number,
}

impl NumberLiteral {
    pub fn new(value: Number) -> Self {
        NumberLiteral { value }
    }
}

impl<T: Container> Expression<T> for NumberLiteral {
    fn evaluate<'a: 'b, 'b>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::from(self.value))
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(NumberLiteral { value: self.value })
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl_deref_for_literal!(NumberLiteral, Number);

macro_rules! impl_number_literal_from {
    ($type:ty) => {
        impl<'a> From<$type> for NumberLiteral {
            fn from(value: $type) -> Self {
                NumberLiteral {
                    value: value.into(),
                }
            }
        }
    };
}
impl_number_literal_from!(u8);
impl_number_literal_from!(u16);
impl_number_literal_from!(u32);
impl_number_literal_from!(u64);
impl_number_literal_from!(usize);
impl_number_literal_from!(u128);
impl_number_literal_from!(i8);
impl_number_literal_from!(i16);
impl_number_literal_from!(i32);
impl_number_literal_from!(i64);
impl_number_literal_from!(i128);
impl_number_literal_from!(isize);
impl_number_literal_from!(f32);
impl_number_literal_from!(f64);
impl_number_literal_from!(Number);

#[derive(Debug)]
pub struct BoolLiteral {
    value: bool,
}

impl BoolLiteral {
    pub fn new(value: bool) -> Self {
        BoolLiteral { value }
    }
}

impl<T: Container> Expression<T> for BoolLiteral {
    fn evaluate<'a: 'b, 'b>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::from(self.value))
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(BoolLiteral { value: self.value })
    }
}

impl Display for BoolLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> From<bool> for BoolLiteral {
    fn from(value: bool) -> Self {
        BoolLiteral {
            value: value.into(),
        }
    }
}

impl_deref_for_literal!(BoolLiteral, bool);

#[derive(Debug)]
pub struct MapLiteral<T: Container> {
    value: HashMap<String, Box<dyn Expression<T>>>,
}

impl<T: Container> MapLiteral<T> {
    pub fn new(value: HashMap<String, Box<dyn Expression<T>>>) -> Self {
        MapLiteral { value }
    }
}

impl<T: Container> Expression<T> for MapLiteral<T> {
    fn evaluate<'a: 'b, 'b>(&'a self, d: &'b T) -> Result<Any<'b>> {
        // Any values that return an error are skipped when building the hash and will
        // fail silently.
        Ok(Any::from(
            self.value
                .iter()
                .filter_map(|(k, v)| Some((k.to_string(), v.evaluate(d).ok()?)))
                .collect::<HashMap<String, Any<'b>>>(),
        ))
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(MapLiteral {
            value: self
                .value
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_ref().clone()))
                .collect(),
        })
    }
}

impl<T: Container> Display for MapLiteral<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T: Container> Deref for MapLiteral<T> {
    type Target = HashMap<String, Box<dyn Expression<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

macro_rules! impl_map_literal_from {
    ($type:ty) => {
        impl<'a, T: Container> From<$type> for MapLiteral<T> {
            fn from(value: $type) -> Self {
                MapLiteral {
                    value: value.into(),
                }
            }
        }
    };
}
impl_map_literal_from!(HashMap<String, Box<dyn Expression<T>>>);
