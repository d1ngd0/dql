use std::fmt::Display;

use crate::{Any, Container, Expr, Number, Result};

use super::Expression;

// Math expressions
macro_rules! impl_expression_math_op {
    ($name:ident, $op:tt) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            left: Box<Expr>,
            right: Box<Expr>,
        }

        impl $name {
            pub fn new(left: Expr, right: Expr) -> Self {
                Self { left: Box::new(left), right: Box::new(right) }
            }
        }

        impl Expression for $name {
            fn evaluate<'a: 'b, 'b, T: Container>(&'a self, d: &'b T) -> Result<Any<'b>> {
                let left: Number = self.left.evaluate(d)?.try_into()?;
                let right: Number = self.right.evaluate(d)?.try_into()?;

                Ok(Any::Number(left $op right))
            }

        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {} {}", self.left, stringify!($op), self.right)
            }
        }
    };
}

impl_expression_math_op!(ModulusExpression, %);
impl_expression_math_op!(DivideExpression, /);
impl_expression_math_op!(MultiplyExpression, *);
impl_expression_math_op!(AddExpression, +);
impl_expression_math_op!(SubtractExpression, -);

#[derive(Debug, Clone)]
pub struct ExponentExpression {
    left: Box<Expr>,
    right: Box<Expr>,
}

impl ExponentExpression {
    pub fn new(left: Expr, right: Expr) -> ExponentExpression {
        ExponentExpression {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl Expression for ExponentExpression {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, d: &'b T) -> Result<Any<'b>> {
        let left: Number = self.left.evaluate(d)?.try_into()?;
        let right: Number = self.right.evaluate(d)?.try_into()?;

        Ok(Any::from(i64::from(left).pow(u32::from(right))))
    }
}

impl Display for ExponentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, stringify!(EXPONENT), self.right)
    }
}

#[derive(Debug, Clone)]
pub struct SubExpression {
    expr: Box<Expr>,
}

impl SubExpression {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }
}

impl Expression for SubExpression {
    fn evaluate<'a: 'b, 'b, T: Container>(&'a self, d: &'b T) -> Result<Any<'b>> {
        self.expr.evaluate(d)
    }
}

impl Display for SubExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expr)
    }
}
