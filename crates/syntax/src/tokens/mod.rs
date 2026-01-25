use crate::Span;
use logos::{Lexer, Logos};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use logos::internal::LexerInternal;
use thiserror::Error;

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

impl<T: PartialEq> PartialEq<T> for SpannedToken<T> {
    fn eq(&self, other: &T) -> bool {
        self.token.eq(other)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Normal(NormalToken<'a>),
    String(StringToken<'a>),
    StringFormat(StringFormatToken<'a>),
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
            Token::Normal(t) => Token::Normal(t.into_owned()),
            Token::String(t) => Token::String(t.into_owned()),
            Token::StringFormat(t) => Token::StringFormat(t.into_owned()),
            Token::Error(err) => Token::Error(err),
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Token::Normal(token) => token.kind(),
            Token::String(token) => token.kind(),
            Token::StringFormat(token) => token.kind(),
            Token::Error(err) => err.kind(),
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Token::Error(_) => true,
            _ => false,
        }
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexerError))]
#[logos(skip r"[ \t\n\f]+")]
pub enum NormalToken<'a> {
    #[regex(r#"\/\/[^\n]*"#, allow_greedy = true, callback = token_cow)]
    SingleLineComment(Cow<'a, str>),
    #[regex(r#"(?m)/\*(.|\n)+?\*/"#, callback = token_cow)]
    MultiLineComment(Cow<'a, str>),
    #[regex(r#"\/\/\/[^\n]*"#, allow_greedy = true, callback = token_cow)]
    DocComment(Cow<'a, str>),

    #[token("mod")]
    KeywordMod,
    #[token("part")]
    KeywordPart,
    #[token("sketch")]
    KeywordSketch,
    #[token("op")]
    KeywordOp,
    #[token("fn")]
    KeywordFn,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("use")]
    KeywordUse,
    #[token("as")]
    KeywordAs,
    #[token("return")]
    KeywordReturn,
    #[token("pub")]
    KeywordPub,
    #[token("const")]
    KeywordConst,
    #[token("prop")]
    KeywordProp,
    #[token("init")]
    KeywordInit,

    #[regex("_*[a-zA-Z][_a-zA-Z0-9-']*", callback = token_cow)]
    Identifier(Cow<'a, str>),

    #[regex(r#"-?(0|[1-9]\d*)"#, callback = token_cow)]
    LiteralInt(Cow<'a, str>),
    #[regex(r#"-?(0|[1-9]\d*)?\.(\d+)((e|E)(-|\+)?(\d+))?"#, callback = token_cow)]
    LiteralFloat(Cow<'a, str>),
    #[token(r#"""#, string_token_callback)]
    String(Vec<SpannedToken<Token<'a>>>),
    #[token("true")]
    LiteralBoolTrue,
    #[token("false")]
    LiteralBoolFalse,

    #[token(":")]
    SigilColon,
    #[token(";")]
    SigilSemiColon,
    #[token("::")]
    SigilDoubleColon,
    #[token("(")]
    SigilOpenBracket,
    #[token(")")]
    SigilCloseBracket,
    #[token("[")]
    SigilOpenSquareBracket,
    #[token("]")]
    SigilCloseSquareBracket,
    #[token("{")]
    SigilOpenCurlyBracket,
    #[token("}")]
    SigilCloseCurlyBracket,
    #[token("#")]
    SigilHash,
    #[token(".")]
    SigilDot,
    #[token(",")]
    SigilComma,
    #[token("..")]
    SigilDoubleDot,
    #[token("@")]
    SigilAt,
    #[token("->")]
    SigilSingleArrow,

    #[token("+")]
    OperatorAdd,
    #[token("-")]
    OperatorSubtract,
    #[token("*")]
    OperatorMultiply,
    #[token("/")]
    OperatorDivide,
    #[token("|")]
    OperatorUnion,
    #[token("&")]
    OperatorIntersect,
    #[token("^")]
    OperatorPowerXor,
    #[token(">")]
    OperatorGreaterThan,
    #[token("<")]
    OperatorLessThan,
    #[token(">=")]
    OperatorGreaterEqual,
    #[token("<=")]
    OperatorLessEqual,
    #[token("~")]
    OperatorNear,
    #[token("==")]
    OperatorEqual,
    #[token("!=")]
    OperatorNotEqual,
    #[token("and")]
    OperatorAnd,
    #[token("or")]
    OperatorOr,
    #[token("xor")]
    OperatorXor,
    #[token("!")]
    OperatorNot,
    #[token("=")]
    OperatorAssignment,
}

impl NormalToken<'_> {
    pub fn into_owned(self) -> NormalToken<'static> {
        match self {
            NormalToken::SingleLineComment(c) => {
                NormalToken::SingleLineComment(c.into_owned().into())
            }
            NormalToken::MultiLineComment(c) => {
                NormalToken::MultiLineComment(c.into_owned().into())
            }
            NormalToken::DocComment(c) => NormalToken::DocComment(c.into_owned().into()),
            NormalToken::Identifier(s) => NormalToken::Identifier(s.into_owned().into()),
            NormalToken::LiteralInt(s) => NormalToken::LiteralInt(s.into_owned().into()),
            NormalToken::LiteralFloat(s) => NormalToken::LiteralFloat(s.into_owned().into()),
            NormalToken::String(s) => {
                NormalToken::String(s.into_iter().map(SpannedToken::into_owned).collect())
            }
            NormalToken::KeywordMod => NormalToken::KeywordMod,
            NormalToken::KeywordPart => NormalToken::KeywordPart,
            NormalToken::KeywordSketch => NormalToken::KeywordSketch,
            NormalToken::KeywordOp => NormalToken::KeywordOp,
            NormalToken::KeywordFn => NormalToken::KeywordFn,
            NormalToken::KeywordIf => NormalToken::KeywordIf,
            NormalToken::KeywordElse => NormalToken::KeywordElse,
            NormalToken::KeywordUse => NormalToken::KeywordUse,
            NormalToken::KeywordAs => NormalToken::KeywordAs,
            NormalToken::KeywordReturn => NormalToken::KeywordReturn,
            NormalToken::KeywordPub => NormalToken::KeywordPub,
            NormalToken::KeywordConst => NormalToken::KeywordConst,
            NormalToken::KeywordProp => NormalToken::KeywordProp,
            NormalToken::KeywordInit => NormalToken::KeywordInit,
            NormalToken::LiteralBoolTrue => NormalToken::LiteralBoolTrue,
            NormalToken::LiteralBoolFalse => NormalToken::LiteralBoolFalse,
            NormalToken::SigilColon => NormalToken::SigilColon,
            NormalToken::SigilSemiColon => NormalToken::SigilSemiColon,
            NormalToken::SigilDoubleColon => NormalToken::SigilDoubleColon,
            NormalToken::SigilOpenBracket => NormalToken::SigilOpenBracket,
            NormalToken::SigilCloseBracket => NormalToken::SigilCloseBracket,
            NormalToken::SigilOpenSquareBracket => NormalToken::SigilOpenSquareBracket,
            NormalToken::SigilCloseSquareBracket => NormalToken::SigilCloseSquareBracket,
            NormalToken::SigilOpenCurlyBracket => NormalToken::SigilOpenCurlyBracket,
            NormalToken::SigilCloseCurlyBracket => NormalToken::SigilCloseCurlyBracket,
            NormalToken::SigilHash => NormalToken::SigilHash,
            NormalToken::SigilDot => NormalToken::SigilDot,
            NormalToken::SigilComma => NormalToken::SigilComma,
            NormalToken::SigilDoubleDot => NormalToken::SigilDoubleDot,
            NormalToken::SigilAt => NormalToken::SigilAt,
            NormalToken::SigilSingleArrow => NormalToken::SigilSingleArrow,
            NormalToken::OperatorAdd => NormalToken::OperatorAdd,
            NormalToken::OperatorSubtract => NormalToken::OperatorSubtract,
            NormalToken::OperatorMultiply => NormalToken::OperatorMultiply,
            NormalToken::OperatorDivide => NormalToken::OperatorDivide,
            NormalToken::OperatorUnion => NormalToken::OperatorUnion,
            NormalToken::OperatorIntersect => NormalToken::OperatorIntersect,
            NormalToken::OperatorPowerXor => NormalToken::OperatorPowerXor,
            NormalToken::OperatorGreaterThan => NormalToken::OperatorGreaterThan,
            NormalToken::OperatorLessThan => NormalToken::OperatorLessThan,
            NormalToken::OperatorGreaterEqual => NormalToken::OperatorGreaterEqual,
            NormalToken::OperatorLessEqual => NormalToken::OperatorLessEqual,
            NormalToken::OperatorNear => NormalToken::OperatorNear,
            NormalToken::OperatorEqual => NormalToken::OperatorEqual,
            NormalToken::OperatorNotEqual => NormalToken::OperatorNotEqual,
            NormalToken::OperatorAnd => NormalToken::OperatorAnd,
            NormalToken::OperatorOr => NormalToken::OperatorOr,
            NormalToken::OperatorXor => NormalToken::OperatorXor,
            NormalToken::OperatorNot => NormalToken::OperatorNot,
            NormalToken::OperatorAssignment => NormalToken::OperatorAssignment,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NormalToken::SingleLineComment(_) => "single-line comment",
            NormalToken::MultiLineComment(_) => "multi-line comment",
            NormalToken::DocComment(_) => "doc comment",
            NormalToken::KeywordMod => "mod",
            NormalToken::KeywordPart => "part",
            NormalToken::KeywordSketch => "sketch",
            NormalToken::KeywordOp => "op",
            NormalToken::KeywordFn => "fn",
            NormalToken::KeywordIf => "if",
            NormalToken::KeywordElse => "else",
            NormalToken::KeywordUse => "use",
            NormalToken::KeywordAs => "as",
            NormalToken::KeywordReturn => "return",
            NormalToken::KeywordPub => "pub",
            NormalToken::KeywordConst => "const",
            NormalToken::KeywordProp => "prop",
            NormalToken::KeywordInit => "init",
            NormalToken::Identifier(_) => "identifier",
            NormalToken::LiteralInt(_) => "integer",
            NormalToken::LiteralFloat(_) => "float",
            NormalToken::String(_) => "string",
            NormalToken::LiteralBoolTrue => "boolean",
            NormalToken::LiteralBoolFalse => "boolean",
            NormalToken::SigilColon => ":",
            NormalToken::SigilSemiColon => ";",
            NormalToken::SigilDoubleColon => "::",
            NormalToken::SigilOpenBracket => "(",
            NormalToken::SigilCloseBracket => ")",
            NormalToken::SigilOpenSquareBracket => "[",
            NormalToken::SigilCloseSquareBracket => "]",
            NormalToken::SigilOpenCurlyBracket => "{",
            NormalToken::SigilCloseCurlyBracket => "}",
            NormalToken::SigilHash => "#",
            NormalToken::SigilDot => ".",
            NormalToken::SigilComma => ",",
            NormalToken::SigilDoubleDot => "..",
            NormalToken::SigilAt => "@",
            NormalToken::SigilSingleArrow => "->",
            NormalToken::OperatorAdd => "+",
            NormalToken::OperatorSubtract => "-",
            NormalToken::OperatorMultiply => "*",
            NormalToken::OperatorDivide => "/",
            NormalToken::OperatorUnion => "|",
            NormalToken::OperatorIntersect => "&",
            NormalToken::OperatorPowerXor => "^",
            NormalToken::OperatorGreaterThan => ">",
            NormalToken::OperatorLessThan => "<",
            NormalToken::OperatorGreaterEqual => ">=",
            NormalToken::OperatorLessEqual => "<=",
            NormalToken::OperatorNear => "!",
            NormalToken::OperatorEqual => "==",
            NormalToken::OperatorNotEqual => "!=",
            NormalToken::OperatorAnd => "and",
            NormalToken::OperatorOr => "or",
            NormalToken::OperatorXor => "xor",
            NormalToken::OperatorNot => "not",
            NormalToken::OperatorAssignment => "=",
        }
    }
}

fn string_token_callback<'a>(
    lex: &mut Lexer<'a, NormalToken<'a>>,
) -> Result<Vec<SpannedToken<Token<'a>>>, LexerError> {
    let mut string_lexer = lex.clone().morph::<StringToken>();
    let mut tokens = Vec::new();
    while let Some(token) = string_lexer.next() {
        match token {
            Ok(StringToken::Quote) => break,
            Err(e) => {
                let start = lex.span().start;
                *lex = string_lexer.morph();
                return Err(match e {
                    LexerError::UnclosedStringFormat(span) => LexerError::UnclosedStringFormat(start..span.end),
                    e => e
                })
            },
            Ok(tok) => tokens.push(SpannedToken {
                span: string_lexer.span(),
                token: Token::String(tok),
            }),
        }
    }
    *lex = string_lexer.morph();
    Ok(tokens)
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexerError))]
pub enum StringToken<'a> {
    #[regex(r#"[^"{}\\]+"#, callback = token_cow)]
    Content(Cow<'a, str>),
    #[regex(r#"\\["\\/bfnrt]"#, callback = token_cow)]
    Escaped(Cow<'a, str>),
    #[token(r#"\"#)]
    BackSlash,
    #[token(r#"{{"#)]
    EscapedCurlyOpen,
    #[token(r#"}}"#)]
    EscapedCurlyClose,
    #[token("{", format_token_callback)]
    FormatStart(Vec<SpannedToken<Token<'a>>>),
    #[token(r#"""#)]
    Quote,
}

impl StringToken<'_> {
    pub fn into_owned(self) -> StringToken<'static> {
        match self {
            StringToken::Content(s) => StringToken::Content(s.into_owned().into()),
            StringToken::Escaped(s) => StringToken::Escaped(s.into_owned().into()),
            StringToken::FormatStart(f) => {
                StringToken::FormatStart(f.into_iter().map(SpannedToken::into_owned).collect())
            }
            StringToken::BackSlash => StringToken::BackSlash,
            StringToken::EscapedCurlyOpen => StringToken::EscapedCurlyOpen,
            StringToken::EscapedCurlyClose => StringToken::EscapedCurlyClose,
            StringToken::Quote => StringToken::Quote,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            StringToken::Content(_) => "string literal",
            StringToken::Escaped(_) => "escaped character",
            StringToken::BackSlash => "\\",
            StringToken::EscapedCurlyOpen => "{",
            StringToken::EscapedCurlyClose => "}",
            StringToken::FormatStart(_) => "string format",
            StringToken::Quote => "\"",
        }
    }
}

/// Check if the string is just a literal without formating
pub fn is_literal_string(string_tokens: &[SpannedToken<Token>]) -> bool {
    !string_tokens
        .iter()
        .any(|token| matches!(token.token, Token::String(StringToken::FormatStart(_))))
}

/// Get the literal value of string tokens, if the string is a literal
pub fn get_literal_string(string_tokens: &[SpannedToken<Token>]) -> Option<String> {
    let mut result = String::new();
    for token in string_tokens {
        match &token.token {
            Token::String(StringToken::Content(s)) => result.push_str(s.as_ref()),
            Token::String(StringToken::Escaped(s)) => result.push_str(&s[1..]),
            Token::String(StringToken::BackSlash) => result.push('\\'),
            Token::String(StringToken::EscapedCurlyOpen) => result.push('{'),
            Token::String(StringToken::EscapedCurlyClose) => result.push('}'),
            _ => return None,
        }
    }

    Some(result)
}

fn format_token_callback<'a>(
    lex: &mut Lexer<'a, StringToken<'a>>,
) -> Result<Vec<SpannedToken<Token<'a>>>, LexerError> {
    let mut expression_lexer = lex.clone().morph::<NormalToken>();
    let mut tokens = Vec::new();

    let mut with_format = false;
    while let Some(token) = expression_lexer.next() {
        match token {
            Ok(NormalToken::SigilCloseCurlyBracket) => break,
            Ok(NormalToken::SigilColon) => {
                with_format = true;
                break;
            }
            Ok(NormalToken::String(content)) => {
                let start = lex.span().start;
                let end = content.first().map(|t| t.span.start).unwrap_or_else(|| expression_lexer.span().start);
                lex.end(end);
                return Err(LexerError::UnclosedStringFormat(start..end))
            },
            Err(e) => return Err(e),
            Ok(tok) => tokens.push(SpannedToken {
                span: expression_lexer.span(),
                token: Token::Normal(tok),
            }),
        }
    }

    let mut format_lexer = expression_lexer.morph::<StringFormatToken>();
    if with_format {
        while let Some(token) = format_lexer.next() {
            match token {
                Ok(StringFormatToken::FormatEnd) => break,
                Err(e) => {
                    *lex = format_lexer.morph();
                    return Err(e)
                },
                Ok(StringFormatToken::StringEnd) => {
                    let start = lex.span().start;
                    let end = format_lexer.span().end;
                    lex.end(end);
                    return Err(LexerError::UnclosedStringFormat(start..end))
                },
                Ok(tok) => tokens.push(SpannedToken {
                    span: format_lexer.span(),
                    token: Token::StringFormat(tok),
                }),
            }
        }
    }


    *lex = format_lexer.morph();
    Ok(tokens)
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexerError))]
pub enum StringFormatToken<'a> {
    #[token("}")]
    FormatEnd,
    #[regex(r#"\.[\d]+"#, callback = token_cow)]
    FormatPrecision(Cow<'a, str>),
    #[regex(r#"0[\d]+"#, callback = token_cow)]
    FormatWidth(Cow<'a, str>),
    #[token("\"")]
    StringEnd,
}

impl StringFormatToken<'_> {
    pub fn into_owned(self) -> StringFormatToken<'static> {
        match self {
            StringFormatToken::FormatPrecision(s) => {
                StringFormatToken::FormatPrecision(s.into_owned().into())
            }
            StringFormatToken::FormatWidth(s) => {
                StringFormatToken::FormatWidth(s.into_owned().into())
            }
            StringFormatToken::FormatEnd => StringFormatToken::FormatEnd,
            StringFormatToken::StringEnd => StringFormatToken::StringEnd,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            StringFormatToken::FormatEnd => "}",
            StringFormatToken::FormatPrecision(_) => "format precision",
            StringFormatToken::FormatWidth(_) => "format width",
            StringFormatToken::StringEnd => "\"",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Error)]
pub enum LexerError {
    #[default]
    #[error("No valid token")]
    NoValidToken,
    #[error("Unclosed format string")]
    UnclosedStringFormat(Span),
}

impl LexerError {
    pub fn kind(&self) -> &'static str {
        match self {
            LexerError::NoValidToken => "no valid token",
            LexerError::UnclosedStringFormat(_) => "unclosed format string",
        }
    }
}

impl LexerError {
    pub fn span(&self) -> Option<Span> {
        match self {
            LexerError::UnclosedStringFormat(span) => Some(span.clone()),
            _ => None,
        }
    }
}

pub fn lex<'a>(input: &'a str) -> Vec<SpannedToken<Token<'a>>> {
    Lexer::<NormalToken>::new(input)
        .spanned()
        .map(|(token, span)| match token {
            Ok(token) => SpannedToken {
                span,
                token: Token::Normal(token),
            },
            Err(error) => SpannedToken { span: error.span().unwrap_or(span), token: Token::Error(error) },
        })
        .collect()
}

fn token_cow<'a, Token: Logos<'a, Source = str>>(lex: &mut Lexer<'a, Token>) -> Cow<'a, str> {
    Cow::Borrowed(lex.slice())
}
