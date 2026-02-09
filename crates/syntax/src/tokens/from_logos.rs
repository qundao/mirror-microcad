// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::tokens::logos::{
    LogosToken, NormalToken, QuoteVariant, StringFormatToken, StringToken, get_literal_string,
};
use crate::tokens::{SpannedToken, Token};
use either::Either;
use logos::SpannedIter;
use std::borrow::Cow;
use std::iter::once;

/// map the lexed tokens from logos into the output tokens, allowing for some post-processing to help the ast building
pub(crate) fn from_logos<'source>(
    tokens: SpannedIter<'source, NormalToken<'source>>,
) -> impl Iterator<Item = SpannedToken<Token<'source>>> {
    tokens
        .map(|(token, span)| match token {
            Ok(token) => SpannedToken {
                span,
                token: LogosToken::Normal(token),
            },
            Err(error) => SpannedToken {
                span: error.span().unwrap_or(span),
                token: LogosToken::Error(error),
            },
        })
        .flat_map(map_token)
}

fn map_token(token: SpannedToken<LogosToken>) -> impl Iterator<Item = SpannedToken<Token>> {
    match token.token {
        LogosToken::Normal(t) => Either::Left(map_normal_token(SpannedToken::new(token.span, t))),
        LogosToken::Error(e) => Either::Right(once(SpannedToken {
            span: token.span,
            token: Token::Error(e),
        })),
    }
}

fn map_normal_token(token: SpannedToken<NormalToken>) -> impl Iterator<Item = SpannedToken<Token>> {
    match token.token {
        NormalToken::SingleLineComment(c) => Either::Left(once(SpannedToken::new(
            token.span,
            Token::SingleLineComment(c),
        ))),
        NormalToken::MultiLineComment(c) => Either::Left(once(SpannedToken::new(
            token.span,
            Token::MultiLineComment(c),
        ))),
        NormalToken::DocComment(c) => {
            Either::Left(once(SpannedToken::new(token.span, Token::DocComment(c))))
        }
        NormalToken::KeywordMod => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordMod)))
        }
        NormalToken::KeywordPart => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordPart)))
        }
        NormalToken::KeywordSketch => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordSketch)))
        }
        NormalToken::KeywordOp => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordOp)))
        }
        NormalToken::KeywordFn => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordFn)))
        }
        NormalToken::KeywordIf => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordIf)))
        }
        NormalToken::KeywordElse => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordElse)))
        }
        NormalToken::KeywordUse => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordUse)))
        }
        NormalToken::KeywordAs => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordAs)))
        }
        NormalToken::KeywordReturn => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordReturn)))
        }
        NormalToken::KeywordPub => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordPub)))
        }
        NormalToken::KeywordConst => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordConst)))
        }
        NormalToken::KeywordProp => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordProp)))
        }
        NormalToken::KeywordInit => {
            Either::Left(once(SpannedToken::new(token.span, Token::KeywordInit)))
        }
        NormalToken::Identifier(i) => {
            Either::Left(once(SpannedToken::new(token.span, Token::Identifier(i))))
        }
        NormalToken::Unit(u) => Either::Left(once(SpannedToken::new(token.span, Token::Unit(u)))),
        NormalToken::LiteralInt(l) => {
            Either::Left(once(SpannedToken::new(token.span, Token::LiteralInt(l))))
        }
        NormalToken::LiteralIntWithRange(l) => Either::Right(Box::new(
            once(SpannedToken::new(
                token.span.start..(token.span.end - 2),
                Token::LiteralInt(match l {
                    Cow::Borrowed(l) => Cow::Borrowed(&l[0..l.len() - 2]),
                    Cow::Owned(mut l) => {
                        l.truncate(l.len() - 2);
                        Cow::Owned(l)
                    }
                }),
            ))
            .chain(once(SpannedToken::new(
                (token.span.end - 2)..token.span.end,
                Token::SigilDoubleDot,
            ))),
        )
            as Box<dyn Iterator<Item = SpannedToken<Token>>>),
        NormalToken::LiteralFloat(l) => {
            Either::Left(once(SpannedToken::new(token.span, Token::LiteralFloat(l))))
        }
        NormalToken::Quote(QuoteVariant::Unit) => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilQuote)))
        }
        NormalToken::Quote(QuoteVariant::String(tokens)) => {
            Either::Right(match get_literal_string(&tokens) {
                Some(literal) => Box::new(once(SpannedToken::new(
                    token.span,
                    Token::LiteralString(literal.into()),
                )))
                    as Box<dyn Iterator<Item = SpannedToken<Token>>>,
                None => Box::new(
                    once(SpannedToken::new(
                        token.span.start..(token.span.start + 1),
                        Token::FormatStringStart,
                    ))
                    .chain(tokens.into_iter().flat_map(map_string_token))
                    .chain(once(SpannedToken::new(
                        (token.span.end - 1)..token.span.end,
                        Token::FormatStringEnd,
                    ))),
                ) as Box<dyn Iterator<Item = SpannedToken<Token>>>,
            })
        }
        NormalToken::LiteralBoolTrue => Either::Left(once(SpannedToken::new(
            token.span,
            Token::LiteralBool(true),
        ))),
        NormalToken::LiteralBoolFalse => Either::Left(once(SpannedToken::new(
            token.span,
            Token::LiteralBool(false),
        ))),
        NormalToken::SigilColon => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilColon)))
        }
        NormalToken::SigilSemiColon => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilSemiColon)))
        }
        NormalToken::SigilDoubleColon => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilDoubleColon)))
        }
        NormalToken::SigilOpenBracket => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilOpenBracket)))
        }
        NormalToken::SigilCloseBracket => Either::Left(once(SpannedToken::new(
            token.span,
            Token::SigilCloseBracket,
        ))),
        NormalToken::SigilOpenSquareBracket => Either::Left(once(SpannedToken::new(
            token.span,
            Token::SigilOpenSquareBracket,
        ))),
        NormalToken::SigilCloseSquareBracket => Either::Left(once(SpannedToken::new(
            token.span,
            Token::SigilCloseSquareBracket,
        ))),
        NormalToken::SigilOpenCurlyBracket => Either::Left(once(SpannedToken::new(
            token.span,
            Token::SigilOpenCurlyBracket,
        ))),
        NormalToken::SigilCloseCurlyBracket => Either::Left(once(SpannedToken::new(
            token.span,
            Token::SigilCloseCurlyBracket,
        ))),
        NormalToken::SigilHash => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilHash)))
        }
        NormalToken::SigilDot => Either::Left(once(SpannedToken::new(token.span, Token::SigilDot))),
        NormalToken::SigilComma => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilComma)))
        }
        NormalToken::SigilDoubleDot => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilDoubleDot)))
        }
        NormalToken::SigilAt => Either::Left(once(SpannedToken::new(token.span, Token::SigilAt))),
        NormalToken::SigilSingleArrow => {
            Either::Left(once(SpannedToken::new(token.span, Token::SigilSingleArrow)))
        }
        NormalToken::OperatorAdd => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorAdd)))
        }
        NormalToken::OperatorSubtract => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorSubtract)))
        }
        NormalToken::OperatorMultiply => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorMultiply)))
        }
        NormalToken::OperatorDivide => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorDivide)))
        }
        NormalToken::OperatorUnion => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorUnion)))
        }
        NormalToken::OperatorIntersect => Either::Left(once(SpannedToken::new(
            token.span,
            Token::OperatorIntersect,
        ))),
        NormalToken::OperatorPowerXor => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorPowerXor)))
        }
        NormalToken::OperatorGreaterThan => Either::Left(once(SpannedToken::new(
            token.span,
            Token::OperatorGreaterThan,
        ))),
        NormalToken::OperatorLessThan => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorLessThan)))
        }
        NormalToken::OperatorGreaterEqual => Either::Left(once(SpannedToken::new(
            token.span,
            Token::OperatorGreaterEqual,
        ))),
        NormalToken::OperatorLessEqual => Either::Left(once(SpannedToken::new(
            token.span,
            Token::OperatorLessEqual,
        ))),
        NormalToken::OperatorNear => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorNear)))
        }
        NormalToken::OperatorEqual => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorEqual)))
        }
        NormalToken::OperatorNotEqual => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorNotEqual)))
        }
        NormalToken::OperatorAnd => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorAnd)))
        }
        NormalToken::OperatorOr => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorOr)))
        }
        NormalToken::OperatorXor => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorXor)))
        }
        NormalToken::OperatorNot => {
            Either::Left(once(SpannedToken::new(token.span, Token::OperatorNot)))
        }
        NormalToken::OperatorAssignment => Either::Left(once(SpannedToken::new(
            token.span,
            Token::OperatorAssignment,
        ))),
    }
}

fn map_string_token(token: SpannedToken<StringToken>) -> impl Iterator<Item = SpannedToken<Token>> {
    match token.token {
        StringToken::Content(c) => {
            Either::Left(once(SpannedToken::new(token.span, Token::StringContent(c))))
        }
        StringToken::Escaped(c) => {
            Either::Left(once(SpannedToken::new(token.span, Token::Character(c))))
        }
        StringToken::BackSlash => {
            Either::Left(once(SpannedToken::new(token.span, Token::Character('\\'))))
        }
        StringToken::EscapedCurlyOpen => {
            Either::Left(once(SpannedToken::new(token.span, Token::Character('}'))))
        }
        StringToken::EscapedCurlyClose => {
            Either::Left(once(SpannedToken::new(token.span, Token::Character('}'))))
        }
        StringToken::FormatStart((expr, format)) => Either::Right(Box::new(
            once(SpannedToken::new(
                token.span.start..(token.span.start + 1),
                Token::StringFormatOpen,
            ))
            .chain(expr.into_iter().flat_map(map_normal_token))
            .chain(format.into_iter().flat_map(map_string_format_token))
            .chain(once(SpannedToken::new(
                (token.span.end - 1)..token.span.end,
                Token::StringFormatClose,
            ))),
        )
            as Box<dyn Iterator<Item = SpannedToken<Token>>>),
        StringToken::Quote => {
            Either::Left(once(SpannedToken::new(token.span, Token::FormatStringEnd)))
        }
    }
}

fn map_string_format_token(
    token: SpannedToken<StringFormatToken>,
) -> impl Iterator<Item = SpannedToken<Token>> {
    match token.token {
        StringFormatToken::FormatEnd => {
            once(SpannedToken::new(token.span, Token::StringFormatClose))
        }
        StringFormatToken::FormatPrecision(c) => once(SpannedToken::new(
            token.span,
            Token::StringFormatPrecision(c),
        )),
        StringFormatToken::FormatWidth(c) => {
            once(SpannedToken::new(token.span, Token::StringFormatWidth(c)))
        }
        StringFormatToken::StringEnd => {
            once(SpannedToken::new(token.span, Token::StringFormatClose))
        }
    }
}
