// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Span;
use crate::ast::{BinaryOperation, Comment, Expression, ItemExtra, ItemExtras, Operator};
use crate::parser::{Error, Extra, ParserInput, STRUCTURAL_TOKENS};
use crate::tokens::Token;
use chumsky::extra::{Full, ParserExtra, SimpleState};
use chumsky::input::Input;
use chumsky::prelude::*;
use chumsky::{IterParser, Parser, extra, select_ref};

pub fn comment_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, Comment, Extra<'tokens>> + 'tokens {
    let single_line_comments = select_ref! {
        Token::SingleLineComment(comment) => comment
    }
    .repeated()
    .at_least(1)
    .collect::<Vec<_>>()
    .map_with(|lines, e| Comment {
        span: e.span(),
        lines: lines.into_iter().map(|s| s.as_ref().into()).collect(),
    })
    .boxed();
    single_line_comments
        .or(select_ref! {
            Token::MultiLineComment(comment) = e => Comment {
                span: e.span(),
                lines: vec![comment.as_ref().into()]
            }
        })
        .labelled("comment")
        .boxed()
}

pub fn extras_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, Vec<ItemExtra>, Extra<'tokens>> {
    comment_parser()
        .map(ItemExtra::Comment)
        .repeated()
        .collect::<Vec<_>>()
}

/// Ignore tokens, until we hit the end of a pair or nested curly brackets
///
/// Used for error recovery
pub fn ignore_till_matched_brackets<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (), Extra<'tokens>> {
    none_of(STRUCTURAL_TOKENS)
        .repeated()
        .ignore_then(nested_delimiters(
            Token::SigilOpenCurlyBracket,
            Token::SigilCloseCurlyBracket,
            [
                (
                    Token::SigilOpenSquareBracket,
                    Token::SigilCloseSquareBracket,
                ),
                (Token::SigilOpenBracket, Token::SigilCloseBracket),
            ],
            |_| (),
        ))
        .boxed()
}

/// Ignore tokens, until we hit a semicolon
///
/// Used for error recovery
pub fn ignore_till_semi<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (), Extra<'tokens>> {
    none_of(Token::SigilSemiColon)
        .repeated()
        .at_least(1)
        .then(just(Token::SigilSemiColon))
        .ignored()
        .boxed()
}

pub fn binop<'tokens, I>(
    params: I,
    tokens: &'static [Token<'static>],
) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, Expression, Extra<'tokens>> + Clone
where
    I: Parser<'tokens, ParserInput<'tokens, 'tokens>, Expression, Extra<'tokens>> + Clone + 'tokens,
{
    params
        .clone()
        .foldl_with(
            one_of(tokens).then(params).repeated(),
            |lhs, (op, rhs), e| {
                Expression::BinaryOperation(BinaryOperation {
                    span: e.span(),
                    lhs: lhs.into(),
                    operation: match op {
                        Token::OperatorAdd => Operator::Add,
                        Token::OperatorSubtract => Operator::Subtract,
                        Token::OperatorMultiply => Operator::Multiply,
                        Token::OperatorDivide => Operator::Divide,
                        Token::OperatorUnion => Operator::Union,
                        Token::OperatorIntersect => Operator::Intersect,
                        Token::OperatorPowerXor => Operator::PowerXor,
                        Token::OperatorGreaterThan => Operator::GreaterThan,
                        Token::OperatorLessThan => Operator::LessThan,
                        Token::OperatorGreaterEqual => Operator::GreaterEqual,
                        Token::OperatorLessEqual => Operator::LessEqual,
                        Token::OperatorNear => Operator::Near,
                        Token::OperatorEqual => Operator::Equal,
                        Token::OperatorNotEqual => Operator::NotEqual,
                        Token::OperatorAnd => Operator::And,
                        Token::OperatorOr => Operator::Or,
                        Token::OperatorXor => Operator::Xor,
                        _ => unreachable!(),
                    },
                    rhs: rhs.into(),
                })
            },
        )
        .boxed()
}

pub trait ParserExt<'src, I, O, E = extra::Default>: Parser<'src, I, O, E>
where
    I: Input<'src, Span = Span>,
    E: ParserExtra<'src, I>,
    O: 'src,
{
    fn with_extras(self) -> impl Parser<'src, I, (O, ItemExtras), E> + 'src;

    fn delimited_with_spanned_error<B, C, U, V, F>(
        self,
        before: B,
        after: C,
        err_map: F,
    ) -> impl Parser<'src, I, O, E>
    where
        B: Parser<'src, I, U, Full<E::Error, SimpleState<Span>, E::Context>>,
        C: Parser<'src, I, V, Full<E::Error, SimpleState<Span>, E::Context>>,
        F: Fn(E::Error, I::Span, I::Span) -> E::Error;
}

impl<'tokens, O, P> ParserExt<'tokens, ParserInput<'tokens, 'tokens>, O, Extra<'tokens>> for P
where
    P: Parser<'tokens, ParserInput<'tokens, 'tokens>, O, Extra<'tokens>> + 'tokens,
    O: 'tokens,
{
    fn with_extras(
        self,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (O, ItemExtras), Extra<'tokens>> {
        extras_parser()
            .then(self)
            .then(extras_parser())
            .map(|((leading, res), trailing)| (res, ItemExtras { leading, trailing }))
            .boxed()
    }

    fn delimited_with_spanned_error<B, C, U, V, F>(
        self,
        before: B,
        after: C,
        err_map: F,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, O, Extra<'tokens>>
    where
        B: Parser<
                'tokens,
                ParserInput<'tokens, 'tokens>,
                U,
                Full<Error<'tokens>, SimpleState<Span>, ()>,
            >,
        C: Parser<
                'tokens,
                ParserInput<'tokens, 'tokens>,
                V,
                Full<Error<'tokens>, SimpleState<Span>, ()>,
            >,
        F: Fn(Error<'tokens>, Span, Span) -> Error<'tokens>,
    {
        before
            .map_with(|_, e| *e.state() = e.span().into())
            .then(self.with_state(()))
            .then(after.map_err_with_state(move |e, span: Span, state| err_map(e, state.0.clone(), span)))
            .map(|((_, res), _)| res)
            .with_state(SimpleState(0..0))
    }
}
