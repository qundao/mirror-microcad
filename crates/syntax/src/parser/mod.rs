mod literal;
mod expression;

use std::num::{ParseFloatError, ParseIntError};
use chumsky::error::Rich;
use chumsky::{extra, Parser};
use chumsky::input::BorrowInput;
use crate::ast::{SourceFile, Statement};
use crate::parser::expression::expression_parser;
use crate::Span;
use crate::tokens::Token;

pub fn parser<'tokens, 'src: 'tokens, I>()
    -> impl Parser<'tokens, I, SourceFile, extra::Err<Rich<'tokens, Token<'src>, Span>>>
    + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    expression_parser().map(|ex| SourceFile {
        span: ex.span(),
        statements: vec![Statement::Expression(ex)],
    })
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFloat(ParseFloatError),
    InvalidInt(ParseIntError),
    Unknown,
}