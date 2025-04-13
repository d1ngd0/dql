use std::{marker::PhantomData, time::Duration};

use crate::Container;

use super::{Error, History, Result, expression::*, lexor::Lexer};

pub const SELECT: &str = "SELECT";
pub const SELECT_SEP: &str = ",";
pub const SELECT_ALIAS: &str = "AS";
pub const FROM: &str = "FROM";
pub const FROM_SEP: &str = ",";
pub const WHERE: &str = "WHERE";
pub const HAVING: &str = "HAVING";
pub const GROUP: &str = "GROUP";
pub const ORDER: &str = "ORDER";
pub const BY: &str = "BY";
pub const ORDER_ASC: &str = "ASC";
pub const ORDER_DESC: &str = "DESC";
pub const LIMIT: &str = "LIMIT";
pub const INTERVAL: &str = "INTERVAL";
pub const EVICT: &str = "EVICT";
pub const EMIT: &str = "EMIT";
pub const ON: &str = "ON";
pub const SUB_CONDITION: &str = "(";
pub const SUB_CONDITION_END: &str = ")";
pub const EQUAL: &str = "=";
pub const EQUAL_DOUBLE: &str = "==";
pub const NOT_EQUAL: &str = "!=";
pub const IN: &str = "IN";
pub const GREATER_THAN: &str = ">";
pub const LESS_THAN: &str = "<";
pub const GREATER_THAN_EQUAL: &str = ">=";
pub const LESS_THAN_EQUAL: &str = "<=";
pub const AND: &str = "AND";
pub const OR: &str = "OR";
pub const NEGATE: &str = "!";
pub const KEY_WRAP: &str = "`";
pub const IDENTIFIER_WRAP: &str = "\"";
pub const STRING_WRAP: &str = "'";
pub const NULL: &str = "NULL";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const MAP_WRAP: &str = "{";
pub const MAP_WRAP_END: &str = "}";
pub const MAP_CHILD_SET: &str = ":";
pub const MAP_CHILD_SEP: &str = ",";
pub const ARRAY_WRAP: &str = "[";
pub const ARRAY_WRAP_END: &str = "]";
pub const ARRAY_CHILD_SEP: &str = ",";

pub const ADD: &str = "+";
pub const MINUS: &str = "-";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";
pub const MODULUS: &str = "%";
pub const EXPONENT: &str = "^";
pub const SUB_EXPR_OPEN: &str = "(";
pub const SUB_EXPR_CLOSE: &str = ")";

pub const FN_OPEN: &str = "(";
pub const FN_CLOSE: &str = ")";
pub const FN_SEP: &str = ",";

pub const FN_EXISTS: &str = "EXISTS";

pub const AGGREGATION_SUM: &str = "SUM";
pub const AGGREGATION_COUNT: &str = "COUNT";
pub const AGGREGATION_AVG: &str = "AVG";

// Parser is used to parse a query string into a query struct, it produces all
// sorts of interior structs as well.
pub struct Parser<'a, T: Container> {
    lex: Lexer<'a>,
    _p: PhantomData<T>,
}

// must_token consumes and returns the next token, if we have run out
// of tokens it will return Error::UnexpectedEOF. this function *is*
// case sensitive
macro_rules! must_token {
    ( $source:ident ) => {
        $source
            .lex
            .token()
            .ok_or_else(|| Error::unexpected_eof($source.history()))
    };
}

// peak is a shortcut for self.lex.peak and it returns the next
// token from the tokenizer without consuming it
macro_rules! peak {
    ( $source:ident ) => {
        $source.lex.peak()
    };
}

// token returns the next token from the tokenizer, it *does* consume
// the token, moving the head forward.
macro_rules! token {
    ( $source:ident ) => {
        $source.lex.token()
    };
}

// consume just consumes the next token, no questions asked.
macro_rules! consume {
    ( $source:ident ) => {{
        let _ = $source.lex.token();
    }};
}

// is_next will check to see if the next value matches the supplied
// token without consuming it. This function is not case sensative
macro_rules! is_next {
    ( $source:ident, $seen:expr ) => {
        peak!($source)
            .map(|v| v.to_uppercase())
            .filter(|v| v == $seen)
            .is_some()
    };
}

// continue_if checks if the tok supplies as an arguement is the next
// token, if so it consumes is and returns true, if it doesn't it
// returns false. This can be usefule for controlling a loop. For instance
// in a `SELECT` clause we want to continue if the next token is a `,` after
// each aggregation. This function *is not* case sensitive
macro_rules! continue_if {
    ( $source:ident, $seen:expr ) => {
        peak!($source)
            .map(|v| v.to_uppercase())
            .filter(|v| v == $seen)
            .inspect(|_| consume!($source))
            .is_some()
    };
}

// consume_next will consume the next token if it is the expected type
// This function is not case sensative. If the token is something else
// an error is returned
macro_rules! consume_next {
    ( $source:ident, $expected:expr ) => {
        must_token!($source).and_then(|v| {
            if v.to_uppercase() == $expected {
                Ok(())
            } else {
                Err(Error::with_history(
                    &format!("expected \"{}\" but got \"{}\"", $expected, v),
                    $source.history(),
                ))
            }
        })
    };
}

// TODO: this is stupid, and we need to change this to a parser
// builder
impl<'a, T: Container> From<&'a str> for Parser<'a, T> {
    fn from(s: &'a str) -> Parser<'a, T> {
        Parser {
            lex: Lexer::from(s),
            _p: PhantomData,
        }
    }
}

impl<'a, T: Container> Parser<'a, T> {
    // consumed returns a History object, which lets the caller know where
    // the head of the lexor is. This is useful for creating error messages
    // since you can point out where problems are
    pub fn history(&self) -> History {
        History::new(self.lex.consumed(), self.lex.future())
    }

    // parse_identifier allows you to parse a string with an optional wrapping
    // token. This should be used for things that could be passed unwrapped or
    // wrapped, like the `FROM` clause values.
    pub fn parse_identifier(&mut self, wrap: &str) -> Result<String> {
        let wrapped = continue_if!(self, wrap);

        let value = must_token!(self)?;

        if wrapped {
            consume_next!(self, wrap)?;
        }

        Ok(String::from(value))
    }

    pub fn parse_duration(&mut self, wrap: &str) -> Result<Duration> {
        let wrapped = continue_if!(self, wrap);
        if wrapped {
            consume!(self);
        }

        let value = must_token!(self)?;

        if wrapped {
            consume_next!(self, wrap)?;
        }

        parse_duration::parse(value)
            .map_err(|err| Error::InvalidQuery(format!("invalid duration {}", err)))
    }

    // expression parses an expression, returning it as a Box<dyn Expression>
    pub fn expression(&mut self) -> Result<Box<dyn Expression<T>>> {
        self.parse_expression_add()
    }

    // parse_expression_add makes it possible to support `Order Of Operations`.
    // This function handles adding and subtracting linearly, and passes lower
    // scopes into the multiply function
    fn parse_expression_add(&mut self) -> Result<Box<dyn Expression<T>>> {
        let mut expr = self.parse_expression_multiply()?;

        loop {
            let next = peak!(self).unwrap_or_default();
            match next {
                ADD => {
                    consume!(self);
                    let right = self.parse_expression_multiply()?;
                    expr = Box::new(AddExpression::new(expr, right))
                }
                MINUS => {
                    consume!(self);
                    let right = self.parse_expression_multiply()?;
                    expr = Box::new(SubtractExpression::new(expr, right))
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // parse_expression_multiply makes it possible to support `Order Of Operations`.
    // This function handles multipling, dividing, remainder linearly, and passes lower
    // scopes into the exponent function
    fn parse_expression_multiply(&mut self) -> Result<Box<dyn Expression<T>>> {
        let mut expr = self.parse_expression_exponent()?;

        loop {
            let next = peak!(self).unwrap_or_default();
            match next {
                MULTIPLY => {
                    consume!(self);
                    let right = self.parse_expression_exponent()?;
                    expr = Box::new(MultiplyExpression::new(expr, right))
                }
                DIVIDE => {
                    consume!(self);
                    let right = self.parse_expression_exponent()?;
                    expr = Box::new(DivideExpression::new(expr, right))
                }
                MODULUS => {
                    consume!(self);
                    let right = self.parse_expression_exponent()?;
                    expr = Box::new(ModulusExpression::new(expr, right))
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // parse_expression_exponent makes it possible to support `Order Of Operations`.
    // This function handles exponents linearly, and passes execution into the
    // parse_expression function
    fn parse_expression_exponent(&mut self) -> Result<Box<dyn Expression<T>>> {
        let mut expr = self.parse_expression()?;

        loop {
            let next = peak!(self).unwrap_or_default();
            match next {
                EXPONENT => {
                    consume!(self);
                    let right = self.parse_expression()?;
                    expr = Box::new(ExponentExpression::new(expr, right))
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // parse_expression is used to parse expressions without evaluating math
    // in other words this handles all the things you would expect `expression`
    // to handle if you didn't have to deal with math.
    fn parse_expression(&mut self) -> Result<Box<dyn Expression<T>>> {
        let left = peak!(self).unwrap_or_default();

        match left.to_uppercase().as_str() {
            SUB_EXPR_OPEN => {
                consume!(self);
                let expr = self.expression()?;
                consume_next!(self, SUB_EXPR_CLOSE)?;
                Ok(Box::new(SubExpression::new(expr)))
            }
            // KEY_WRAP => Ok(Box::new(PathExpression::from_parser(self)?)),
            // STRING_WRAP => Ok(Box::new(StringExpression::from_parser(self)?)),
            // MAP_WRAP => Ok(Box::new(MapLiteral::from_parser(self)?)),
            // ARRAY_WRAP => Ok(Box::new(ArrayLiteral::from_parser(self)?)),
            // FN_LOWER => Ok(Box::new(StringLower::from_parser(self)?)),
            // FN_UPPER => Ok(Box::new(StringUpper::from_parser(self)?)),
            // FN_LENGTH => Ok(Box::new(StringLength::from_parser(self)?)),
            // FN_TRIM => Ok(Box::new(StringTrim::from_parser(self)?)),
            // FN_TRIM_LEFT => Ok(Box::new(StringTrimLeft::from_parser(self)?)),
            // FN_TRIM_RIGHT => Ok(Box::new(StringTrimRight::from_parser(self)?)),
            // FN_CONCAT => Ok(Box::new(StringConcat::from_parser(self)?)),
            // FN_SPLIT => Ok(Box::new(StringSplit::from_parser(self)?)),
            // TRUE => Ok(Box::new(BoolExpression::from_parser(self)?)),
            // FALSE => Ok(Box::new(BoolExpression::from_parser(self)?)),
            NULL => Ok(Box::new(self.null()?)),
            //_ => self.parse_unwrapped_expression(),
            _ => todo!(),
        }
    }

    // fn parse_unwrapped_expression(&mut self) -> Result<Box<dyn Expression>> {
    //     let left = self.peak().unwrap_or_default();
    //     let mut chars = left.chars();
    //     match chars.next() {
    //         Some('0'..='9') | Some('-') => Ok(Box::new(NumberExpression::from_parser(self)?)),
    //         _ => Ok(Box::new(PathExpression::from_parser(self)?)),
    //     }
    // }

    fn null(&mut self) -> Result<NullExpression<T>> {
        consume_next!(self, NULL)?;
        Ok(NullExpression::default())
    }
}
