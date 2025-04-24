use std::fmt::Display;

use crate::Any;

use super::{Expr, Expression};

#[derive(Expression, Clone, Debug)]
pub struct ToUpper {
    value: Expr,
}

impl Display for ToUpper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Expression for ToUpper {
    fn evaluate<'a: 'b, 'b, T: crate::Container>(
        &'a self,
        c: &'b T,
    ) -> crate::Result<crate::Any<'b>> {
        let v = self.value.evaluate(c)?;
        Ok(Any::from(v.as_str()?.to_uppercase()))
    }
}
