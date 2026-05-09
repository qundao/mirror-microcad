// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Chumsky parser functions for µcad syntax elements

use crate::ast;

use crate::parser::{ParserInput, RichError, helpers::ParserExt};
use crate::tokens::Token;
use chumsky::extra::Full;
use chumsky::inspector::Inspector;
use chumsky::{IterParser, Parser, select_ref};

/// Alias for parser input type
pub type PInput<'a> = ParserInput<'a, 'a>;

/// Alias for parser error type
pub type PError<'a, S, Ctx> = Full<RichError<'a>, S, Ctx>;

/// Alias for Inspector type (as a trait bound shorthand)
pub trait PInspector<'a>: Inspector<'a, PInput<'a>> + Default + Clone + 'static {}

impl<'a, T> PInspector<'a> for T where T: Inspector<'a, PInput<'a>> + Default + Clone + 'static {}

/// Unit parser.
pub fn unit<'tokens, S, Ctx>()
-> impl Parser<'tokens, PInput<'tokens>, ast::Unit, PError<'tokens, S, Ctx>>
where
    S: PInspector<'tokens>,
    Ctx: 'tokens,
{
    select_ref! {
        Token::Identifier(ident) = e => ast::Unit {
            span: e.span(),
            name: ident.as_ref().into()
        },
        Token::Unit(unit) = e => ast::Unit {
            span: e.span(),
            name: unit.as_ref().into()
        },
        Token::SigilQuote = e => ast::Unit {
            span: e.span(),
            name: r#"""#.into()
        },
    }
    .labelled("unit")
}

/// Literal parser.
pub fn literal<'tokens, S, Ctx>()
-> impl Parser<'tokens, PInput<'tokens>, ast::Literal, PError<'tokens, S, Ctx>>
where
    S: PInspector<'tokens>,
    Ctx: 'tokens,
{
    use microcad_lang_base::ToCompactString;
    use std::str::FromStr;

    {
        let single_value = select_ref! {
            Token::LiteralFloat(x) = e => {
                match f64::from_str(x) {
                    Ok(value) => ast::LiteralKind::Float(ast::FloatLiteral {
                        value,
                        raw: x.to_compact_string(),
                        span: e.span(),
                    }),
                    Err(err) => ast::LiteralKind::Error(ast::LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    })
                }
            },
            Token::LiteralInt(x) = e => {
                match i64::from_str(x) {
                    Ok(value) => ast::LiteralKind::Integer(ast::IntegerLiteral {
                    value,
                    raw: x.to_compact_string(),
                    span: e.span(),
                }),
                    Err(err) => ast::LiteralKind::Error(ast::LiteralError {
                        span: e.span(),
                        kind: err.into(),
                    })
                }
            },
            Token::LiteralString(content) = e => {
                ast::LiteralKind::String(ast::StringLiteral {
                    span: e.span(),
                    content: content.as_ref().into(),
                })
            },
            Token::LiteralBool(value) = e => {
                ast::LiteralKind::Bool(ast::BoolLiteral {
                    span: e.span(),
                    value: *value,
                })
            },
        }
        .boxed();

        single_value
            .then(unit().or_not())
            .with_extras()
            .try_map_with(|((literal, ty), extras), e| {
                let literal = match (literal, ty) {
                    (ast::LiteralKind::Float(float), Some(unit)) => {
                        ast::LiteralKind::Quantity(ast::QuantityLiteral {
                            span: e.span(),
                            value: float.value,
                            raw: float.raw,
                            unit,
                        })
                    }
                    (ast::LiteralKind::Integer(int), Some(unit)) => {
                        ast::LiteralKind::Quantity(ast::QuantityLiteral {
                            span: e.span(),
                            value: int.value as f64,
                            raw: int.raw,
                            unit,
                        })
                    }
                    (_, Some(_)) => ast::LiteralKind::Error(ast::LiteralError {
                        span: e.span(),
                        kind: ast::LiteralErrorKind::Untypable,
                    }),
                    (literal, None) => literal,
                };
                Ok(ast::Literal {
                    span: e.span(),
                    literal,
                    extras,
                })
            })
            .labelled("literal")
            .boxed()
    }
}

/// Comment parser.
pub fn comment<'tokens, S, Ctx>()
-> impl Parser<'tokens, PInput<'tokens>, ast::Comment, PError<'tokens, S, Ctx>>
where
    S: PInspector<'tokens>,
    Ctx: 'tokens,
{
    let single_line_comments = select_ref! {
        Token::SingleLineComment(comment) => comment
    }
    .map_with(|line, e| ast::Comment {
        span: e.span(),
        inner: ast::CommentInner::SingleLine(line.to_string()),
    })
    .boxed();
    let multi_line = select_ref! {
        Token::MultiLineComment(comment) = e => ast::Comment {
            span: e.span(),
            inner: ast::CommentInner::MultiLine(comment.to_string())
        }
    };

    single_line_comments
        .or(multi_line)
        .labelled("comment")
        .boxed()
}

/// Whitespace parser.
pub fn whitespace<'tokens, S, Ctx>()
-> impl Parser<'tokens, PInput<'tokens>, String, PError<'tokens, S, Ctx>> + 'tokens + Clone
where
    S: PInspector<'tokens>,
    Ctx: 'tokens,
{
    select_ref! {
        Token::Whitespace(s) => s.to_string(),
    }
    .labelled("whitespace")
    .boxed()
}

/// Leading extras parser.
pub fn leading_extras<'tokens, S, Ctx>()
-> impl Parser<'tokens, PInput<'tokens>, ast::LeadingExtras, PError<'tokens, S, Ctx>>
where
    S: PInspector<'tokens>,
    Ctx: 'tokens,
{
    // Inline the whitespace logic and the comment logic
    let whitespace =
        select_ref! { Token::Whitespace(s) => ast::ItemExtra::Whitespace(s.to_string()) };
    let comment = comment().map(ast::ItemExtra::Comment);

    comment
        .or(whitespace)
        .repeated()
        .collect::<Vec<_>>()
        .map(ast::LeadingExtras)
        .boxed()
}

/// Trailing extras parser.
pub fn trailing_extras<'tokens, S, Ctx>()
-> impl Parser<'tokens, PInput<'tokens>, ast::TrailingExtras, PError<'tokens, S, Ctx>>
where
    S: PInspector<'tokens>,
    Ctx: 'tokens,
{
    // Inline the whitespace logic and the comment logic
    let whitespace =
        select_ref! { Token::Whitespace(s) => ast::ItemExtra::Whitespace(s.to_string()) };
    let comment = comment().map(ast::ItemExtra::Comment);

    whitespace
        .or(comment)
        .repeated()
        .collect::<Vec<_>>()
        .map(ast::TrailingExtras)
        .boxed()
}
