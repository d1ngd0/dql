use std::fmt::{Debug, Display};

pub trait Container: Debug + Display + Sync + Send {}
