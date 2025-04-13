mod literals;
mod math;

pub use literals::*;
pub use math::*;
use std::fmt::Display;

use crate::{Any, Container, error::Result};

// Expression is a trait that takes in a dapt packet and returns an
// optional value. This value can be Any type, which is what a dapt packet
// can return.
pub trait Expression<T>: Display + Send + Sync
where
    T: Container,
{
    fn evaluate<'a, 'b>(&'a self, c: &'b T) -> Result<Any<'b>>;

    fn clone(&self) -> Box<dyn Expression<T>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::Parser;
    use serde_json::Value;

    impl Container for Value {}

    macro_rules! assert_expression {
        ( $source:expr, $expr:expr, $expected:expr) => {
            let mut parser = Parser::from($expr);
            let expr = parser.expression().unwrap();
            let d: Value = serde_json::from_str($source).unwrap();
            let result = expr.evaluate(&d).unwrap();
            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_expression() {
        assert_expression!(r#"{}"#, "NULL", Any::Null);
        // assert_expression!(r#"{"a.b.c": "hello"}"#, r#"length("\"a.b.c\"")"#, 5);
    }
}
