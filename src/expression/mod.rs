mod literals;
mod math;
mod string;

pub use literals::*;
pub use math::*;
use std::fmt::{Debug, Display};
pub use string::*;

use crate::{Any, Container, error::Result};

// Expression is a trait that takes in a dapt packet and returns an
// optional value. This value can be Any type, which is what a dapt packet
// can return.
pub trait Expression: Display + Debug + Send + Sync + Clone {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, c: &'b T) -> Result<Any<'b>>;
}

macro_rules! expr_impl {
    ($( $i:ident ),* ) => {
        #[derive(Debug, Clone)]
        pub enum Expr {
            $( $i($i), )*
        }

        impl Expr {
            fn evaluate<'a: 'b, 'b, T: Container>(&'a self, c: &'b T) -> Result<Any<'b>> {
                match self {
                    $( Expr::$i(expr) => expr.evaluate(c), )*
                }
            }
        }

        impl Display for Expr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

                match self {
                    $( Expr::$i(expr) => Display::fmt(expr, f), )*
                }
            }
        }

        $(
            impl From<$i> for Expr {
                fn from(val: $i) -> Self {
                    Expr::$i(val)
                }
            }
        )*
    };
}

expr_impl!(
    StringLiteral,
    NumberLiteral,
    MapLiteral,
    ListLiteral,
    BoolLiteral,
    NullExpression,
    ModulusExpression,
    DivideExpression,
    MultiplyExpression,
    AddExpression,
    SubtractExpression,
    SubExpression,
    ExponentExpression
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Str, parser::Parser};
    use serde_json::Value;

    impl Container for Value {}

    macro_rules! assert_expression {
        ( $source:expr, $expr:expr, $expected:expr) => {
            let mut parser = Parser::from($expr);
            let expr = parser.expression()?;
            let d: Any = serde_json::from_str($source).unwrap();
            let result = expr.evaluate(&d)?;
            let result = serde_json::to_string(&result).unwrap();
            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_expression() -> Result<()> {
        assert_expression!(r#"{}"#, "NULL", "null");
        assert_expression!(r#"{}"#, "''", "\"\"");
        assert_expression!(r#"{}"#, "'hello'", "\"hello\"");
        assert_expression!(r#"{}"#, "10", "10");
        assert_expression!(r#"{}"#, "10+25", "35");
        assert_expression!(r#"{}"#, "25/2", "12");
        assert_expression!(r#"{}"#, "25.0/2", "12.5");
        assert_expression!(r#"{}"#, "25.0-2", "23.0");
        assert_expression!(r#"{}"#, "25.0^2", "625");
        assert_expression!(r#"{}"#, "25.0*2", "50.0");
        assert_expression!(r#"{}"#, "25.0*2", "50.0");
        assert_expression!(r#"{}"#, "34-66*11+(45^2)/10.0", "-489.5");
        assert_expression!(r#"{}"#, "true", "true");
        assert_expression!(r#"{}"#, "false", "false");
        assert_expression!(r#"{}"#, "{'something': true}", r#"{"something":true}"#);
        assert_expression!(r#"{}"#, "['something', 12]", r#"["something",12]"#);

        Ok(())
    }
}
