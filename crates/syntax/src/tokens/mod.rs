use crate::Span;
use crate::tokens::from_logos::from_logos;
use crate::tokens::logos::NormalToken;
use ::logos::Lexer;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use thiserror::Error;

mod from_logos;
mod logos;

#[derive(Debug, PartialEq, Clone)]
pub struct SpannedToken<T> {
    pub span: Span,
    pub token: T,
}

impl SpannedToken<Token<'_>> {
    pub fn into_owned(self) -> SpannedToken<Token<'static>> {
        SpannedToken {
            span: self.span,
            token: self.token.into_owned(),
        }
    }
}

impl<T> SpannedToken<T> {
    pub fn new(span: Span, token: T) -> Self {
        SpannedToken { span, token }
    }

    pub fn map_token<U, F: Fn(T) -> U>(self, f: F) -> SpannedToken<U> {
        SpannedToken {
            span: self.span,
            token: f(self.token),
        }
    }
}

impl<T: PartialEq> PartialEq<T> for SpannedToken<T> {
    fn eq(&self, other: &T) -> bool {
        self.token.eq(other)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Error)]
pub enum LexerError {
    #[default]
    #[error("No valid token")]
    NoValidToken,
    #[error("Unclosed format string")]
    UnclosedStringFormat(Span),
    #[error("Unclosed string")]
    UnclosedString(Span),
}

impl LexerError {
    pub fn kind(&self) -> &'static str {
        match self {
            LexerError::NoValidToken => "no valid token",
            LexerError::UnclosedStringFormat(_) => "unclosed format string",
            LexerError::UnclosedString(_) => "unclosed string",
        }
    }
}

impl LexerError {
    pub fn span(&self) -> Option<Span> {
        match self {
            LexerError::UnclosedStringFormat(span) => Some(span.clone()),
            LexerError::UnclosedString(span) => Some(span.clone()),
            _ => None,
        }
    }
}

pub fn lex<'a>(input: &'a str) -> Vec<SpannedToken<Token<'a>>> {
    from_logos(Lexer::<NormalToken>::new(input).spanned()).collect()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    SingleLineComment(Cow<'a, str>),
    MultiLineComment(Cow<'a, str>),
    DocComment(Cow<'a, str>),

    KeywordMod,
    KeywordPart,
    KeywordSketch,
    KeywordOp,
    KeywordFn,
    KeywordIf,
    KeywordElse,
    KeywordUse,
    KeywordAs,
    KeywordReturn,
    KeywordPub,
    KeywordConst,
    KeywordProp,
    KeywordInit,

    Identifier(Cow<'a, str>),
    Unit(Cow<'a, str>),

    LiteralInt(Cow<'a, str>),
    LiteralFloat(Cow<'a, str>),
    LiteralBool(bool),
    LiteralString(Cow<'a, str>),

    FormatStringStart,
    FormatStringEnd,
    StringContent(Cow<'a, str>),
    Character(char),
    StringFormatOpen,
    StringFormatClose,
    StringFormatPrecision(Cow<'a, str>),
    StringFormatWidth(Cow<'a, str>),

    SigilColon,
    SigilSemiColon,
    SigilDoubleColon,
    SigilOpenBracket,
    SigilCloseBracket,
    SigilOpenSquareBracket,
    SigilCloseSquareBracket,
    SigilOpenCurlyBracket,
    SigilCloseCurlyBracket,
    SigilHash,
    SigilDot,
    SigilComma,
    SigilDoubleDot,
    SigilAt,
    SigilSingleArrow,
    SigilQuote,

    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
    OperatorUnion,
    OperatorIntersect,
    OperatorPowerXor,
    OperatorGreaterThan,
    OperatorLessThan,
    OperatorGreaterEqual,
    OperatorLessEqual,
    OperatorNear,
    OperatorEqual,
    OperatorNotEqual,
    OperatorAnd,
    OperatorOr,
    OperatorXor,
    OperatorNot,
    OperatorAssignment,

    Error(LexerError),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind())
    }
}

impl Token<'_> {
    pub fn into_owned(self) -> Token<'static> {
        match self {
            Token::SingleLineComment(c) => Token::SingleLineComment(c.into_owned().into()),
            Token::MultiLineComment(c) => Token::MultiLineComment(c.into_owned().into()),
            Token::DocComment(c) => Token::DocComment(c.into_owned().into()),
            Token::Identifier(s) => Token::Identifier(s.into_owned().into()),
            Token::Unit(s) => Token::Unit(s.into_owned().into()),
            Token::LiteralInt(s) => Token::LiteralInt(s.into_owned().into()),
            Token::LiteralFloat(s) => Token::LiteralFloat(s.into_owned().into()),
            Token::LiteralString(s) => Token::LiteralString(s.into_owned().into()),

            Token::KeywordMod => Token::KeywordMod,
            Token::KeywordPart => Token::KeywordPart,
            Token::KeywordSketch => Token::KeywordSketch,
            Token::KeywordOp => Token::KeywordOp,
            Token::KeywordFn => Token::KeywordFn,
            Token::KeywordIf => Token::KeywordIf,
            Token::KeywordElse => Token::KeywordElse,
            Token::KeywordUse => Token::KeywordUse,
            Token::KeywordAs => Token::KeywordAs,
            Token::KeywordReturn => Token::KeywordReturn,
            Token::KeywordPub => Token::KeywordPub,
            Token::KeywordConst => Token::KeywordConst,
            Token::KeywordProp => Token::KeywordProp,
            Token::KeywordInit => Token::KeywordInit,
            Token::LiteralBool(l) => Token::LiteralBool(l),
            Token::SigilColon => Token::SigilColon,
            Token::SigilSemiColon => Token::SigilSemiColon,
            Token::SigilDoubleColon => Token::SigilDoubleColon,
            Token::SigilOpenBracket => Token::SigilOpenBracket,
            Token::SigilCloseBracket => Token::SigilCloseBracket,
            Token::SigilOpenSquareBracket => Token::SigilOpenSquareBracket,
            Token::SigilCloseSquareBracket => Token::SigilCloseSquareBracket,
            Token::SigilOpenCurlyBracket => Token::SigilOpenCurlyBracket,
            Token::SigilCloseCurlyBracket => Token::SigilCloseCurlyBracket,
            Token::SigilHash => Token::SigilHash,
            Token::SigilDot => Token::SigilDot,
            Token::SigilComma => Token::SigilComma,
            Token::SigilDoubleDot => Token::SigilDoubleDot,
            Token::SigilAt => Token::SigilAt,
            Token::SigilSingleArrow => Token::SigilSingleArrow,
            Token::OperatorAdd => Token::OperatorAdd,
            Token::OperatorSubtract => Token::OperatorSubtract,
            Token::OperatorMultiply => Token::OperatorMultiply,
            Token::OperatorDivide => Token::OperatorDivide,
            Token::OperatorUnion => Token::OperatorUnion,
            Token::OperatorIntersect => Token::OperatorIntersect,
            Token::OperatorPowerXor => Token::OperatorPowerXor,
            Token::OperatorGreaterThan => Token::OperatorGreaterThan,
            Token::OperatorLessThan => Token::OperatorLessThan,
            Token::OperatorGreaterEqual => Token::OperatorGreaterEqual,
            Token::OperatorLessEqual => Token::OperatorLessEqual,
            Token::OperatorNear => Token::OperatorNear,
            Token::OperatorEqual => Token::OperatorEqual,
            Token::OperatorNotEqual => Token::OperatorNotEqual,
            Token::OperatorAnd => Token::OperatorAnd,
            Token::OperatorOr => Token::OperatorOr,
            Token::OperatorXor => Token::OperatorXor,
            Token::OperatorNot => Token::OperatorNot,
            Token::OperatorAssignment => Token::OperatorAssignment,
            Token::FormatStringStart => Token::FormatStringStart,
            Token::FormatStringEnd => Token::FormatStringEnd,
            Token::StringContent(s) => Token::StringContent(s.into_owned().into()),
            Token::Character(c) => Token::Character(c),
            Token::StringFormatOpen => Token::StringFormatOpen,
            Token::StringFormatClose => Token::StringFormatClose,
            Token::SigilQuote => Token::SigilQuote,
            Token::Error(e) => Token::Error(e),
            Token::StringFormatPrecision(c) => Token::StringFormatPrecision(c.into_owned().into()),
            Token::StringFormatWidth(c) => Token::StringFormatWidth(c.into_owned().into()),
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Token::SingleLineComment(_) => "single-line comment",
            Token::MultiLineComment(_) => "multi-line comment",
            Token::DocComment(_) => "doc comment",
            Token::KeywordMod => "mod",
            Token::KeywordPart => "part",
            Token::KeywordSketch => "sketch",
            Token::KeywordOp => "op",
            Token::KeywordFn => "fn",
            Token::KeywordIf => "if",
            Token::KeywordElse => "else",
            Token::KeywordUse => "use",
            Token::KeywordAs => "as",
            Token::KeywordReturn => "return",
            Token::KeywordPub => "pub",
            Token::KeywordConst => "const",
            Token::KeywordProp => "prop",
            Token::KeywordInit => "init",
            Token::Identifier(_) => "identifier",
            Token::Unit(_) => "unit",
            Token::LiteralInt(_) => "integer literal",
            Token::LiteralFloat(_) => "float literal",
            Token::LiteralBool(_) => "boolean literal",
            Token::LiteralString(_) => "string literal",
            Token::FormatStringStart => "start of string",
            Token::FormatStringEnd => "end of string",
            Token::StringContent(_) => "string content",
            Token::Character(_) => "escaped character",
            Token::StringFormatOpen => "string format start",
            Token::StringFormatClose => "string format end",
            Token::SigilColon => ":",
            Token::SigilSemiColon => ";",
            Token::SigilDoubleColon => "::",
            Token::SigilOpenBracket => "(",
            Token::SigilCloseBracket => ")",
            Token::SigilOpenSquareBracket => "[",
            Token::SigilCloseSquareBracket => "]",
            Token::SigilOpenCurlyBracket => "{",
            Token::SigilCloseCurlyBracket => "}",
            Token::SigilHash => "#",
            Token::SigilDot => ".",
            Token::SigilComma => ",",
            Token::SigilDoubleDot => "..",
            Token::SigilAt => "@",
            Token::SigilSingleArrow => "->",
            Token::SigilQuote => "\"",
            Token::OperatorAdd => "+",
            Token::OperatorSubtract => "-",
            Token::OperatorMultiply => "*",
            Token::OperatorDivide => "/",
            Token::OperatorUnion => "|",
            Token::OperatorIntersect => "&",
            Token::OperatorPowerXor => "^",
            Token::OperatorGreaterThan => ">",
            Token::OperatorLessThan => "<",
            Token::OperatorGreaterEqual => ">=",
            Token::OperatorLessEqual => "<=",
            Token::OperatorNear => "!",
            Token::OperatorEqual => "==",
            Token::OperatorNotEqual => "!=",
            Token::OperatorAnd => "and",
            Token::OperatorOr => "or",
            Token::OperatorXor => "xor",
            Token::OperatorNot => "not",
            Token::OperatorAssignment => "=",
            Token::StringFormatPrecision(_) => "format precision",
            Token::StringFormatWidth(_) => "format width",
            Token::Error(e) => e.kind(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Token::Error(_))
    }
}
