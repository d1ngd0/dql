use std::fmt::Display;

use crate::{Any, Container, Number, Result};

use super::Expression;

// Math expressions
macro_rules! impl_expression_math_op {
    ($name:ident, $op:tt) => {
        pub struct $name<T: Container> {
            left: Box<dyn Expression<T>>,
            right: Box<dyn Expression<T>>,
        }

        impl<T: Container> $name<T> {
            pub fn new(left: Box<dyn Expression<T>>, right: Box<dyn Expression<T>>) -> Self {
                Self { left, right }
            }
        }

        impl<T: Container> Expression<T> for $name<T> {
            fn evaluate<'a: 'b, 'b>(&'a self, d: &'b T) -> Result<Any<'b>> {
                let left: Number = self.left.evaluate(d)?.try_into()?;
                let right: Number = self.right.evaluate(d)?.try_into()?;

                Ok(Any::Number(left $op right))
            }

            fn clone(&self) -> Box<dyn Expression<T>> {
                Box::new(Self {
                    left: self.left.clone(),
                    right: self.right.clone(),

                })
            }
        }

        impl<T: Container> Display for $name<T> {
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

pub struct ExponentExpression<T: Container> {
    left: Box<dyn Expression<T>>,
    right: Box<dyn Expression<T>>,
}

impl<T: Container> ExponentExpression<T> {
    pub fn new(
        left: Box<dyn Expression<T>>,
        right: Box<dyn Expression<T>>,
    ) -> ExponentExpression<T> {
        ExponentExpression { left, right }
    }
}

impl<T: Container> Expression<T> for ExponentExpression<T> {
    fn evaluate<'a: 'b, 'b>(&'a self, d: &'b T) -> Result<Any<'b>> {
        let left: Number = self.left.evaluate(d)?.try_into()?;
        let right: Number = self.right.evaluate(d)?.try_into()?;

        Ok(Any::from(i64::from(left).pow(u32::from(right))))
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(Self {
            left: self.left.clone(),
            right: self.right.clone(),
        })
    }
}

impl<T: Container> Display for ExponentExpression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, stringify!(EXPONENT), self.right)
    }
}

pub struct SubExpression<T: Container> {
    expr: Box<dyn Expression<T>>,
}

impl<T: Container> SubExpression<T> {
    pub fn new(expr: Box<dyn Expression<T>>) -> Self {
        Self { expr }
    }
}

impl<T: Container> Expression<T> for SubExpression<T> {
    fn evaluate<'a: 'b, 'b>(&'a self, d: &'b T) -> Result<Any<'b>> {
        self.expr.evaluate(d)
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(Self {
            expr: self.expr.clone(),
        })
    }
}

impl<T: Container> Display for SubExpression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expr)
    }
}
