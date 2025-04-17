mod literals;
mod math;

pub use literals::*;
pub use math::*;
use std::fmt::{Debug, Display};

use crate::{Any, Container, error::Result};

// Expression is a trait that takes in a dapt packet and returns an
// optional value. This value can be Any type, which is what a dapt packet
// can return.
pub trait Expression<T>: Display + Debug + Send + Sync
where
    T: Container,
{
    fn evaluate<'a: 'b, 'b>(&'a self, c: &'b T) -> Result<Any<'b>>;

    fn clone(&self) -> Box<dyn Expression<T>>;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use crate::{Str, parser::Parser};
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
        assert_expression!(r#"{}"#, "''", Any::from(""));
        assert_expression!(r#"{}"#, "'hello'", Any::from("hello"));
        assert_expression!(r#"{}"#, "10", Any::from(10));
        assert_expression!(r#"{}"#, "10+25", Any::from(35));
        assert_expression!(r#"{}"#, "25/2", Any::from(12));
        assert_expression!(r#"{}"#, "25.0/2", Any::from(12.5));
        assert_expression!(r#"{}"#, "25.0-2", Any::from(23));
        assert_expression!(r#"{}"#, "25.0^2", Any::from(625));
        assert_expression!(r#"{}"#, "25.0*2", Any::from(50));
        assert_expression!(r#"{}"#, "25.0*2", Any::from(50));
        assert_expression!(r#"{}"#, "34-66*11+(45^2)/10.0", Any::from(-489.5));
        assert_expression!(r#"{}"#, "true", Any::from(true));
        assert_expression!(r#"{}"#, "false", Any::from(false));
        assert_expression!(
            r#"{}"#,
            "{'something': true}",
            Any::from([(Str::from("something"), Any::from(true))])
        );
        assert_expression!(
            r#"{}"#,
            "['something', 12]",
            Any::from([Any::from("something"), Any::from(12)])
        );
    }
}
