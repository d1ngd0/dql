use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{Any, Container, Result};

use super::Expression;

// NullExpression is an expression that returns a null value.
#[derive(Debug)]
pub struct NullExpression<T> {
    _phantom: PhantomData<T>,
}

impl<T: Container> Default for NullExpression<T> {
    fn default() -> Self {
        NullExpression {
            _phantom: PhantomData,
        }
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
