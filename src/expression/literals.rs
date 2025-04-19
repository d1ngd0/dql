use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::{Any, Container, Number, Result, Str, parser::STRING_WRAP};

use super::{Expr, Expression};

// NullExpression is an expression that returns a null value.
#[derive(Debug, Clone)]
pub struct NullExpression {}

impl Default for NullExpression {
    fn default() -> Self {
        NullExpression {}
    }
}

impl Expression for NullExpression {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::Null)
    }
}

impl Display for NullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

// StringExpression makes a literal string an expression.
#[derive(Debug, Clone)]
pub struct StringLiteral {
    value: String,
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        StringLiteral { value }
    }

    pub fn to_owned(self) -> String {
        self.value
    }
}

impl Expression for StringLiteral {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::from(&self.value))
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
#[derive(Debug, Clone)]
pub struct NumberLiteral {
    value: Number,
}

impl NumberLiteral {
    pub fn new(value: Number) -> Self {
        NumberLiteral { value }
    }

    pub fn to_owned(self) -> Number {
        self.value
    }
}

impl Expression for NumberLiteral {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::from(self.value))
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

#[derive(Debug, Clone)]
pub struct BoolLiteral {
    value: bool,
}

impl BoolLiteral {
    pub fn new(value: bool) -> Self {
        BoolLiteral { value }
    }

    pub fn to_owned(self) -> bool {
        self.value
    }
}

impl Expression for BoolLiteral {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::from(self.value))
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

#[derive(Debug, Clone)]
pub struct MapLiteral {
    value: HashMap<String, Expr>,
}

impl MapLiteral {
    pub fn to_owned(self) -> HashMap<String, Expr> {
        self.value
    }
}

impl MapLiteral {
    pub fn new(value: HashMap<String, Expr>) -> Self {
        MapLiteral { value }
    }
}

impl Expression for MapLiteral {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, c: &'b T) -> Result<Any<'b>> {
        // Any values that return an error are skipped when building the hash and will
        // fail silently.
        Ok(Any::from(
            self.value
                .iter()
                .filter_map(|(k, v)| Some((Str::from(k.as_str()), v.evaluate(c).ok()?)))
                .collect::<HashMap<Str<'a>, Any<'b>>>(),
        ))
    }
}

impl Display for MapLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Deref for MapLiteral {
    type Target = HashMap<String, Expr>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

macro_rules! impl_map_literal_from {
    ($type:ty) => {
        impl From<$type> for MapLiteral {
            fn from(value: $type) -> Self {
                MapLiteral {
                    value: value.into(),
                }
            }
        }
    };
}
impl_map_literal_from!(HashMap<String, Expr>);

#[derive(Debug, Clone)]
pub struct ListLiteral {
    value: Vec<Expr>,
}

impl ListLiteral {
    pub fn to_owned(self) -> Vec<Expr> {
        self.value
    }

    pub fn new(value: Vec<Expr>) -> Self {
        ListLiteral { value }
    }
}

impl Expression for ListLiteral {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, c: &'b T) -> Result<Any<'b>> {
        // Any values that return an error are skipped when building the hash and will
        // fail silently.
        Ok(Any::from(
            self.value
                .iter()
                .filter_map(|v| Some(v.evaluate(c).ok()?))
                .collect::<Vec<Any<'b>>>(),
        ))
    }
}

impl Display for ListLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Deref for ListLiteral {
    type Target = Vec<Expr>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

macro_rules! impl_list_literal_from {
    ($type:ty) => {
        impl From<$type> for ListLiteral {
            fn from(value: $type) -> Self {
                ListLiteral {
                    value: value.into(),
                }
            }
        }
    };
}
impl_list_literal_from!(Vec<Expr>);
