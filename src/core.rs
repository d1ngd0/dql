use crate::Number;
use std::{collections::HashMap, fmt::Debug, fmt::Display};

// Any defines all the data types that the query language can support
pub enum Any<'a> {
    Str(Str<'a>),
    Number(Number),
    Bool(bool),
    List(Vec<Any<'a>>),
    Map(HashMap<String, Any<'a>>),
}

pub enum Str<'a> {
    String(String),
    Str(&'a str),
}

impl<'a> Str<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(str) => str.as_str(),
            Self::Str(str) => str,
        }
    }

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
}
