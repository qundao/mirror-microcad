mod literal;
mod expression;
mod statement;

use std::num::{ParseFloatError, ParseIntError};
use chumsky::error::Rich;
use chumsky::{extra, IterParser, Parser};
use chumsky::input::BorrowInput;
use crate::ast::{SourceFile, Statement};
use crate::parser::statement::{statement_list_parser, statement_parser};
use crate::Span;
use crate::tokens::Token;

pub fn parser<'tokens, 'src: 'tokens, I>()
    -> impl Parser<'tokens, I, SourceFile, extra::Err<Rich<'tokens, Token<'src>, Span>>>
    + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    statement_list_parser().map_with(|statements, ex| SourceFile {
        span: ex.span(),
        statements,
    })
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFloat(ParseFloatError),
    InvalidInt(ParseIntError),
    Unknown,
}