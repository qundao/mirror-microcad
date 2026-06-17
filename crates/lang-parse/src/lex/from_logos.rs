// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lex::Token;
use crate::lex::logos::{
    LogosToken, NormalToken, QuoteVariant, StringFormatToken, StringToken, get_literal_string,
};
use either::Either;
use logos::SpannedIter;
use microcad_lang_base::Spanned;
use std::borrow::Cow;
use std::iter::once;

/// map the lexed tokens from logos into the output tokens, allowing for some post-processing to help the ast building
pub(crate) fn from_logos<'source>(
    tokens: SpannedIter<'source, NormalToken<'source>>,
) -> impl Iterator<Item = Spanned<Token<'source>>> {
    tokens
        .map(|(token, span)| match token {
            Ok(token) => Spanned {
                span,
                value: LogosToken::Normal(token),
            },
            Err(error) => Spanned {
                span: error.span().unwrap_or(span),
                value: LogosToken::Error(error),
            },
        })
        .flat_map(map_token)
}

fn map_token(token: Spanned<LogosToken>) -> impl Iterator<Item = Spanned<Token>> {
    match token.value {
        LogosToken::Normal(t) => Either::Left(map_normal_token(Spanned::new(token.span, t))),
        LogosToken::Error(e) => Either::Right(once(Spanned {
            span: token.span,
            value: Token::Error(e),
        })),
    }
}

fn map_normal_token(token: Spanned<NormalToken>) -> impl Iterator<Item = Spanned<Token>> {
    match token.value {
        NormalToken::Whitespace(c) => {
            Either::Left(once(Spanned::new(token.span, Token::Whitespace(c))))
        }
        NormalToken::SingleLineComment(c) => {
            Either::Left(once(Spanned::new(token.span, Token::SingleLineComment(c))))
        }
        NormalToken::MultiLineComment(c) => {
            Either::Left(once(Spanned::new(token.span, Token::MultiLineComment(c))))
        }
        NormalToken::DocComment(c) => {
            Either::Left(once(Spanned::new(token.span, Token::DocComment(c))))
        }
        NormalToken::InnerDocComment(c) => {
            Either::Left(once(Spanned::new(token.span, Token::InnerDocComment(c))))
        }
        NormalToken::KeywordMod => Either::Left(once(Spanned::new(token.span, Token::KeywordMod))),
        NormalToken::KeywordPart => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordPart)))
        }
        NormalToken::KeywordSketch => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordSketch)))
        }
        NormalToken::KeywordOp => Either::Left(once(Spanned::new(token.span, Token::KeywordOp))),
        NormalToken::KeywordFn => Either::Left(once(Spanned::new(token.span, Token::KeywordFn))),
        NormalToken::KeywordIf => Either::Left(once(Spanned::new(token.span, Token::KeywordIf))),
        NormalToken::KeywordElse => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordElse)))
        }
        NormalToken::KeywordUse => Either::Left(once(Spanned::new(token.span, Token::KeywordUse))),
        NormalToken::KeywordAs => Either::Left(once(Spanned::new(token.span, Token::KeywordAs))),
        NormalToken::KeywordReturn => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordReturn)))
        }
        NormalToken::KeywordPub => Either::Left(once(Spanned::new(token.span, Token::KeywordPub))),
        NormalToken::KeywordConst => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordConst)))
        }
        NormalToken::KeywordProp => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordProp)))
        }
        NormalToken::KeywordInit => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordInit)))
        }
        NormalToken::KeywordPlugin => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordPlugin)))
        }
        NormalToken::KeywordAssembly => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordAssembly)))
        }
        NormalToken::KeywordMaterial => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordMaterial)))
        }
        NormalToken::KeywordUnit => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordUnit)))
        }
        NormalToken::KeywordEnum => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordEnum)))
        }
        NormalToken::KeywordStruct => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordStruct)))
        }
        NormalToken::KeywordMatch => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordMatch)))
        }
        NormalToken::KeywordType => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordType)))
        }
        NormalToken::KeywordExtern => {
            Either::Left(once(Spanned::new(token.span, Token::KeywordExtern)))
        }
        NormalToken::Identifier(i) => {
            Either::Left(once(Spanned::new(token.span, Token::Identifier(i))))
        }
        NormalToken::Unit(u) => Either::Left(once(Spanned::new(token.span, Token::Unit(u)))),
        NormalToken::LiteralInt(l) => {
            Either::Left(once(Spanned::new(token.span, Token::LiteralInt(l))))
        }
        NormalToken::LiteralIntWithRange(l) => Either::Right(Box::new(
            once(Spanned::new(
                token.span.start..(token.span.end - 2),
                Token::LiteralInt(match l {
                    Cow::Borrowed(l) => Cow::Borrowed(&l[0..l.len() - 2]),
                    Cow::Owned(mut l) => {
                        l.truncate(l.len() - 2);
                        Cow::Owned(l)
                    }
                }),
            ))
            .chain(once(Spanned::new(
                (token.span.end - 2)..token.span.end,
                Token::SigilDoubleDot,
            ))),
        )
            as Box<dyn Iterator<Item = Spanned<Token>>>),
        NormalToken::LiteralFloat(l) => {
            Either::Left(once(Spanned::new(token.span, Token::LiteralFloat(l))))
        }
        NormalToken::Quote(QuoteVariant::Unit) => {
            Either::Left(once(Spanned::new(token.span, Token::SigilQuote)))
        }
        NormalToken::Quote(QuoteVariant::String { span, contents }) => {
            Either::Right(match get_literal_string(&contents) {
                Some(literal) => Box::new(once(Spanned::new(
                    span,
                    Token::LiteralString(literal.into()),
                ))) as Box<dyn Iterator<Item = Spanned<Token>>>,
                None => Box::new(
                    once(Spanned::new(
                        span.start..(span.start + 1),
                        Token::FormatStringStart,
                    ))
                    .chain(contents.into_iter().flat_map(map_string_token))
                    .chain(once(Spanned::new(
                        (span.end - 1)..span.end,
                        Token::FormatStringEnd,
                    ))),
                ) as Box<dyn Iterator<Item = Spanned<Token>>>,
            })
        }
        NormalToken::LiteralBoolTrue => {
            Either::Left(once(Spanned::new(token.span, Token::LiteralBool(true))))
        }
        NormalToken::LiteralBoolFalse => {
            Either::Left(once(Spanned::new(token.span, Token::LiteralBool(false))))
        }
        NormalToken::SigilColon => Either::Left(once(Spanned::new(token.span, Token::SigilColon))),
        NormalToken::SigilSemiColon => {
            Either::Left(once(Spanned::new(token.span, Token::SigilSemiColon)))
        }
        NormalToken::SigilDoubleColon => {
            Either::Left(once(Spanned::new(token.span, Token::SigilDoubleColon)))
        }
        NormalToken::SigilOpenBracket => {
            Either::Left(once(Spanned::new(token.span, Token::SigilOpenBracket)))
        }
        NormalToken::SigilCloseBracket => {
            Either::Left(once(Spanned::new(token.span, Token::SigilCloseBracket)))
        }
        NormalToken::SigilOpenSquareBracket => Either::Left(once(Spanned::new(
            token.span,
            Token::SigilOpenSquareBracket,
        ))),
        NormalToken::SigilCloseSquareBracket => Either::Left(once(Spanned::new(
            token.span,
            Token::SigilCloseSquareBracket,
        ))),
        NormalToken::SigilOpenCurlyBracket => {
            Either::Left(once(Spanned::new(token.span, Token::SigilOpenCurlyBracket)))
        }
        NormalToken::SigilCloseCurlyBracket => Either::Left(once(Spanned::new(
            token.span,
            Token::SigilCloseCurlyBracket,
        ))),
        NormalToken::SigilHash => Either::Left(once(Spanned::new(token.span, Token::SigilHash))),
        NormalToken::SigilDot => Either::Left(once(Spanned::new(token.span, Token::SigilDot))),
        NormalToken::SigilComma => Either::Left(once(Spanned::new(token.span, Token::SigilComma))),
        NormalToken::SigilDoubleDot => {
            Either::Left(once(Spanned::new(token.span, Token::SigilDoubleDot)))
        }
        NormalToken::SigilAt => Either::Left(once(Spanned::new(token.span, Token::SigilAt))),
        NormalToken::SigilSingleArrow => {
            Either::Left(once(Spanned::new(token.span, Token::SigilSingleArrow)))
        }
        NormalToken::OperatorAdd => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorAdd)))
        }
        NormalToken::OperatorSubtract => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorSubtract)))
        }
        NormalToken::OperatorMultiply => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorMultiply)))
        }
        NormalToken::OperatorDivide => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorDivide)))
        }
        NormalToken::OperatorUnion => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorUnion)))
        }
        NormalToken::OperatorIntersect => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorIntersect)))
        }
        NormalToken::OperatorPowerXor => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorPowerXor)))
        }
        NormalToken::OperatorGreaterThan => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorGreaterThan)))
        }
        NormalToken::OperatorLessThan => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorLessThan)))
        }
        NormalToken::OperatorGreaterEqual => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorGreaterEqual)))
        }
        NormalToken::OperatorLessEqual => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorLessEqual)))
        }
        NormalToken::OperatorNear => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorNear)))
        }
        NormalToken::OperatorEqual => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorEqual)))
        }
        NormalToken::OperatorNotEqual => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorNotEqual)))
        }
        NormalToken::OperatorAnd => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorAnd)))
        }
        NormalToken::OperatorOr => Either::Left(once(Spanned::new(token.span, Token::OperatorOr))),
        NormalToken::OperatorXor => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorXor)))
        }
        NormalToken::OperatorNot => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorNot)))
        }
        NormalToken::OperatorAssignment => {
            Either::Left(once(Spanned::new(token.span, Token::OperatorAssignment)))
        }
    }
}

fn map_string_token(token: Spanned<StringToken>) -> impl Iterator<Item = Spanned<Token>> {
    match token.value {
        StringToken::Content(c) => {
            Either::Left(once(Spanned::new(token.span, Token::StringContent(c))))
        }
        StringToken::Escaped(c) => {
            Either::Left(once(Spanned::new(token.span, Token::Character(c))))
        }
        StringToken::BackSlash => {
            Either::Left(once(Spanned::new(token.span, Token::Character('\\'))))
        }
        StringToken::EscapedCurlyOpen => {
            Either::Left(once(Spanned::new(token.span, Token::Character('}'))))
        }
        StringToken::EscapedCurlyClose => {
            Either::Left(once(Spanned::new(token.span, Token::Character('}'))))
        }
        StringToken::FormatStart((expr, format)) => Either::Right(Box::new(
            once(Spanned::new(
                token.span.start..(token.span.start + 1),
                Token::StringFormatOpen,
            ))
            .chain(expr.into_iter().flat_map(map_normal_token))
            .chain(format.into_iter().flat_map(map_string_format_token))
            .chain(once(Spanned::new(
                (token.span.end - 1)..token.span.end,
                Token::StringFormatClose,
            ))),
        )
            as Box<dyn Iterator<Item = Spanned<Token>>>),
        StringToken::Quote => Either::Left(once(Spanned::new(token.span, Token::FormatStringEnd))),
    }
}

fn map_string_format_token(
    token: Spanned<StringFormatToken>,
) -> impl Iterator<Item = Spanned<Token>> {
    match token.value {
        StringFormatToken::FormatEnd => once(Spanned::new(token.span, Token::StringFormatClose)),
        StringFormatToken::FormatPrecision(c) => {
            once(Spanned::new(token.span, Token::StringFormatPrecision(c)))
        }
        StringFormatToken::FormatWidth(c) => {
            once(Spanned::new(token.span, Token::StringFormatWidth(c)))
        }
        StringFormatToken::StringEnd => once(Spanned::new(token.span, Token::StringFormatClose)),
    }
}
