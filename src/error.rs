use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

// Error is the Error type for DQL
#[derive(Debug)]
pub enum Error {
    InvalidType,
    InvalidQuery(String),
    ExpressionError(String),
    UnexpectedEOF,
}

impl Error {
    pub fn with_history(msg: &str, history: History<'_>) -> Self {
        if history.1.is_empty() {
            Error::InvalidQuery(format!("[ {} ] {}", history.0, msg))
        } else {
            Error::InvalidQuery(format!("[ {} █ {} ]: {}", history.0, history.1, msg))
        }
    }

    pub fn unexpected_eof(history: History<'_>) -> Self {
        Error::InvalidQuery(format!("unexpected EOF at: \"{}\"", history))
    }
}

// History is used to wrap the content the lexor has already consumed. By making
// this a type it is more likely that a developer in the future won't supply something
// other than that, causing confusing error messages.
pub struct History<'a>(&'a str, &'a str);

impl Display for History<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> History<'a> {
    pub fn new(past: &'a str, future: &'a str) -> Self {
        Self(past.trim_end(), future.trim_start())
    }
}
