// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Span;
use crate::ast::{BinaryOperation, Comment, Expression, ItemExtra, ItemExtras, Operator};
use crate::parser::{Error, Extra, ParserInput, STRUCTURAL_TOKENS};
use crate::tokens::Token;
use chumsky::extra::{Full, ParserExtra, SimpleState};
use chumsky::input::Input;
use chumsky::inspector::Inspector;
use chumsky::prelude::*;
use chumsky::{IterParser, Parser, extra, select_ref};

pub fn comment_parser<'tokens, S, Ctx>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, Comment, Full<Error<'tokens>, S, Ctx>> + 'tokens
where
    S: Inspector<'tokens, ParserInput<'tokens, 'tokens>> + Default + Clone + 'static,
    Ctx: 'tokens,
{
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
    let multi_line = select_ref! {
        Token::MultiLineComment(comment) = e => Comment {
            span: e.span(),
            lines: vec![comment.as_ref().into()]
        }
    };
    let comment = single_line_comments
        .or(multi_line)
        .labelled("comment")
        .boxed();

    whitespace_parser().or_not().ignore_then(comment)
}
pub fn whitespace_parser<'tokens, S, Ctx>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (), Full<Error<'tokens>, S, Ctx>>
+ 'tokens
+ Clone
where
    S: Inspector<'tokens, ParserInput<'tokens, 'tokens>> + Default + Clone + 'static,
    Ctx: 'tokens,
{
    select_ref! {
        Token::Whitespace(_) => ()
    }
    .labelled("whitespace")
    .boxed()
}

pub fn extras_parser<'tokens, S, Ctx>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, Vec<ItemExtra>, Full<Error<'tokens>, S, Ctx>>
where
    S: Inspector<'tokens, ParserInput<'tokens, 'tokens>> + Default + Clone + 'static,
    Ctx: 'tokens,
{
    comment_parser()
        .map(ItemExtra::Comment)
        .repeated()
        .collect::<Vec<_>>()
        .boxed()
}

/// Ignore tokens, until we hit the end of a pair or nested curly brackets
///
/// Used for error recovery
pub fn ignore_till_matched_curly<'tokens>()
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

/// Ignore tokens, until we hit a semicolon, without consuming the semicolon
///
/// Used for error recovery
pub fn ignore_till_semi<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (), Extra<'tokens>> {
    none_of(Token::SigilSemiColon)
        .repeated()
        .at_least(1)
        .then(just(Token::SigilSemiColon).rewind())
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
            whitespace_parser()
                .or_not()
                .ignore_then(one_of(tokens))
                .then_maybe_whitespace()
                .then(params)
                .repeated(),
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

/// Error recovery parser for places where a single significant token is expected.
///
/// Matches anything but a semicolon or whitespace,
/// if a semicolon or whitespace is encountered, no tokens will be consumed
pub fn recovery_expect_any<'tokens, S, Ctx>()
-> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (), Full<Error<'tokens>, S, Ctx>>
+ 'tokens
+ Clone
where
    S: Inspector<'tokens, ParserInput<'tokens, 'tokens>> + Default + Clone + 'static,
    Ctx: 'tokens,
{
    recovery_expect_any_except(&[])
}

/// Same as `recovery_expect_any` but excluding certain tokens
pub fn recovery_expect_any_except<'tokens, S, Ctx>(
    except: &'tokens [Token<'tokens>],
) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (), Full<Error<'tokens>, S, Ctx>>
+ 'tokens
+ Clone
where
    S: Inspector<'tokens, ParserInput<'tokens, 'tokens>> + Default + Clone + 'static,
    Ctx: 'tokens,
{
    none_of(Token::SigilSemiColon)
        .filter(|t: &Token| t.kind() != "whitespace" && !except.contains(t))
        .ignored()
        .or(one_of(Token::SigilSemiColon)
            .ignored()
            .or(one_of(except).ignored())
            .or(whitespace_parser())
            .rewind())
}

pub trait ParserExt<'src, I, O, E = extra::Default>: Parser<'src, I, O, E>
where
    I: Input<'src, Span = Span>,
    E: ParserExtra<'src, I>,
    O: 'src,
{
    fn with_extras(self) -> impl Parser<'src, I, (O, ItemExtras), E> + 'src;

    /// Required a whitespace
    fn then_whitespace(self) -> impl Parser<'src, I, O, E> + 'src;

    /// Allow a whitespace
    fn then_maybe_whitespace(self) -> impl Parser<'src, I, O, E> + 'src;

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

impl<'tokens, O, P, S, Ctx>
    ParserExt<'tokens, ParserInput<'tokens, 'tokens>, O, Full<Error<'tokens>, S, Ctx>> for P
where
    P: Parser<'tokens, ParserInput<'tokens, 'tokens>, O, Full<Error<'tokens>, S, Ctx>> + 'tokens,
    O: 'tokens,
    S: Inspector<'tokens, ParserInput<'tokens, 'tokens>> + Default + Clone + 'static,
    Ctx: 'tokens,
{
    fn with_extras(
        self,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, (O, ItemExtras), Full<Error<'tokens>, S, Ctx>>
    {
        extras_parser()
            .then_maybe_whitespace()
            .then(self)
            .then(extras_parser())
            .map(|((leading, res), trailing)| (res, ItemExtras { leading, trailing }))
            .boxed()
    }

    fn then_whitespace(
        self,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, O, Full<Error<'tokens>, S, Ctx>> {
        self.then_ignore(whitespace_parser())
    }

    fn then_maybe_whitespace(
        self,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, O, Full<Error<'tokens>, S, Ctx>> {
        self.then_ignore(whitespace_parser().or_not())
    }

    fn delimited_with_spanned_error<B, C, U, V, F>(
        self,
        before: B,
        after: C,
        err_map: F,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'tokens>, O, Full<Error<'tokens>, S, Ctx>>
    where
        B: Parser<
                'tokens,
                ParserInput<'tokens, 'tokens>,
                U,
                Full<Error<'tokens>, SimpleState<Span>, Ctx>,
            >,
        C: Parser<
                'tokens,
                ParserInput<'tokens, 'tokens>,
                V,
                Full<Error<'tokens>, SimpleState<Span>, Ctx>,
            >,
        F: Fn(Error<'tokens>, Span, Span) -> Error<'tokens>,
    {
        before
            .map_with(|_, e| *e.state() = e.span().into())
            .then(self.with_state(S::default()))
            .then(
                after.map_err_with_state(move |e, span: Span, state| {
                    err_map(e, state.0.clone(), span)
                }),
            )
            .map(|((_, res), _)| res)
            .with_state(SimpleState(0..0))
    }
}
