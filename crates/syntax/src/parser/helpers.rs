// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Span;
use crate::ast::{BinaryOperation, Comment, Expression, ItemExtra, ItemExtras, Operator};
use crate::parser::{Extra, ParserInput};
use crate::tokens::Token;
use chumsky::extra::ParserExtra;
use chumsky::input::Input;
use chumsky::prelude::one_of;
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
}
