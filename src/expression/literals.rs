use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{Any, Container, Parser, Result};

use super::Expression;

// NullExpression is an expression that returns a null value.
#[derive(Debug)]
pub struct NullExpression<T> {
    _phantom: PhantomData<T>,
}

impl<T: Container> NullExpression<T> {
    pub fn from_parser(parser: &mut Parser<T>) -> Result<Self> {
        parser.consume_next("NULL")?;
        Ok(NullExpression {
            _phantom: PhantomData,
        })
    }
}

impl<T: Container> Expression<T> for NullExpression<T> {
    fn evaluate<'a, 'b>(&'a self, _: &'b T) -> Result<Any<'b>> {
        Ok(Any::Null)
    }

    fn clone(&self) -> Box<dyn Expression<T>> {
        Box::new(NullExpression {
            _phantom: PhantomData,
        })
    }
}

impl<T: Container> Display for NullExpression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
