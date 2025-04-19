mod literals;
mod math;

pub use literals::*;
pub use math::*;
use std::fmt::{Debug, Display};

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
            let expr = parser.expression().unwrap();
            let d: Any = serde_json::from_str($source).unwrap();
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
