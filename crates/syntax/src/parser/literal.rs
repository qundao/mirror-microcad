use chumsky::error::Rich;
use chumsky::{extra, select_ref, Parser};
use chumsky::input::BorrowInput;
use crate::ast::{BoolLiteral, IntegerLiteral, Literal, QuantityLiteral};
use crate::parser::ParseError;
use crate::Span;
use crate::tokens::{Token};
use chumsky::prelude::*;

pub fn literal_parser<'tokens, 'src: 'tokens, I>()
    -> impl Parser<'tokens, I, Result<Literal, ParseError>, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    let single_value = select_ref! {
        Token::LiteralFloat(x) = e => {
            x.parse()
            .map_err(ParseError::InvalidFloat)
            .map(|value| QuantityLiteral {
                value,
                span: e.span(),
                ty: None,
            })
            .map(Literal::Quantity)
        },
        Token::LiteralInt(x) = e => {
            x.parse()
            .map_err(ParseError::InvalidInt)
            .map(|value| IntegerLiteral {
                value,
                span: e.span(),
            })
            .map(Literal::Integer)
        },
        Token::LiteralBoolTrue = e => {
            Ok(Literal::Bool(BoolLiteral {
                span: e.span(),
                value: true,
            }))
        },
        Token::LiteralBoolFalse = e => {
            Ok(Literal::Bool(BoolLiteral {
                span: e.span(),
                value: false,
            }))
        },
    };

    single_value
}

#[test]
fn test_parser() {
    use crate::tokens::{lex, SpannedToken};

    let tokens = lex("10").unwrap();
    let input = tokens.as_slice().map(2..2, |spanned: &SpannedToken<Token>| (&spanned.token, &spanned.span));
    assert_eq!(literal_parser().parse(input).into_result(), Ok(Ok(Literal::Integer(IntegerLiteral {
        value: 10,
        span: 0..2,
    }))));
}
