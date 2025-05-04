use super::{Expr, Expression};
use crate::Any;
use dql_derive::Function;

#[derive(Function, Clone, Debug)]
#[function(name = "to_upper")]
pub struct ToUpper {
    value: Expr,
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
