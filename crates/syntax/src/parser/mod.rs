mod literal;
mod expression;

use std::num::{ParseFloatError, ParseIntError};



#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFloat(ParseFloatError),
    InvalidInt(ParseIntError),
    Unknown,
}