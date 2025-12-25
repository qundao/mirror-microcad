use crate::Span;
use crate::ast::{BoolLiteral, IntegerLiteral, Literal, LiteralError, QuantityLiteral};
use crate::tokens::Token;
use chumsky::error::Rich;
use chumsky::input::BorrowInput;
use chumsky::prelude::*;
use chumsky::{Parser, extra, select_ref};
use std::str::FromStr;

pub fn literal_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Literal, extra::Err<Rich<'tokens, Token<'src>, Span>>> + Clone
where
    I: BorrowInput<'tokens, Token = Token<'src>, Span = Span>,
{
    let single_value = select_ref! {
        Token::LiteralFloat(x) = e => {
            match f64::from_str(x) {
                Ok(value) => Literal::Quantity(QuantityLiteral {
                value,
                span: e.span(),
                ty: None,
            }),
                Err(err) => Literal::Error(LiteralError {
                    span: e.span(),
                    kind: err.into(),
                })
            }
        },
        Token::LiteralInt(x) = e => {
            match i64::from_str(x) {
                Ok(value) => Literal::Integer(IntegerLiteral {
                value,
                span: e.span(),
            }),
                Err(err) => Literal::Error(LiteralError {
                    span: e.span(),
                    kind: err.into(),
                })
            }
        },
        Token::LiteralBoolTrue = e => {
            Literal::Bool(BoolLiteral {
                span: e.span(),
                value: true,
            })
        },
        Token::LiteralBoolFalse = e => {
            Literal::Bool(BoolLiteral {
                span: e.span(),
                value: false,
            })
        },
    };

    single_value.labelled("literal")
}

#[test]
fn test_parser() {
    use crate::tokens::{SpannedToken, lex};

    let tokens = lex("10").unwrap();
    let input = tokens
        .as_slice()
        .map(2..2, |spanned: &SpannedToken<Token>| {
            (&spanned.token, &spanned.span)
        });
    assert_eq!(
        literal_parser().parse(input).into_result(),
        Ok(Literal::Integer(IntegerLiteral {
            value: 10,
            span: 0..2,
        }))
    );
}
