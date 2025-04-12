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
