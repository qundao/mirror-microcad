use crate::Span;
use logos::{Lexer, Logos};

#[derive(Debug, PartialEq, Clone)]
pub struct SpannedToken<T> {
    pub span: Span,
    pub token: T,
}

impl<T: PartialEq> PartialEq<T> for SpannedToken<T> {
    fn eq(&self, other: &T) -> bool {
        self.token.eq(other)
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexerError))]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    #[regex(r#"\/\/[^\n]*"#, allow_greedy = true)]
    SingleLineComment(&'a str),
    #[regex(r#"(?m)/\*(.|\n)+?\*/"#)]
    MultiLineComment(&'a str),
    #[regex(r#"\/\/\/[^\n]*"#, allow_greedy = true)]
    DocComment(&'a str),

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

    #[regex("_*[a-zA-Z][_a-zA-Z0-9-']*")]
    Identifier(&'a str),

    #[regex(r#"-?(0|[1-9]\d*)"#)]
    LiteralInt(&'a str),
    #[regex(r#"-?(0|[1-9]\d*)?\.(\d+)((e|E)(-|\+)?(\d+))?"#)]
    LiteralFloat(&'a str),
    #[token(r#"""#, string_token_callback)]
    String(Vec<SpannedToken<StringToken<'a>>>),
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

fn string_token_callback<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Option<Vec<SpannedToken<StringToken<'a>>>> {
    let mut string_lexer = lex.clone().morph::<StringToken>();
    let mut tokens = Vec::new();
    while let Some(token) = string_lexer.next() {
        match token {
            Ok(StringToken::Quote) => break,
            Err(_) => return None,
            Ok(tok) => tokens.push(SpannedToken {
                span: string_lexer.span(),
                token: tok,
            }),
        }
    }
    *lex = string_lexer.morph();
    Some(tokens)
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum StringToken<'a> {
    #[regex(r#"[^"{}\\]+"#)]
    Content(&'a str),
    #[regex(r#"\\["\\/bfnrt]"#)]
    Escaped(&'a str),
    #[token(r#"\"#)]
    BackSlash,
    #[token(r#"{{"#)]
    EscapedCurlyOpen,
    #[token(r#"}}"#)]
    EscapedCurlyClose,
    #[token("{", format_token_callback)]
    FormatStart(
        (
            Vec<SpannedToken<Token<'a>>>,
            Vec<SpannedToken<StringFormatToken<'a>>>,
        ),
    ),
    #[token(r#"""#)]
    Quote,
}

/// Check if the string is just a literal without formating
pub fn is_literal_string(string_tokens: &[SpannedToken<StringToken>]) -> bool {
    !string_tokens
        .iter()
        .any(|token| matches!(token.token, StringToken::FormatStart(_)))
}

/// Get the literal value of string tokens, if the string is a literal
pub fn get_literal_string(string_tokens: &[SpannedToken<StringToken>]) -> Option<String> {
    let mut result = String::new();
    for token in string_tokens {
        match token.token {
            StringToken::Content(s) => result.push_str(s),
            StringToken::Escaped(s) => result.push_str(&s[1..]),
            StringToken::BackSlash => result.push('\\'),
            StringToken::EscapedCurlyOpen => result.push('{'),
            StringToken::EscapedCurlyClose => result.push('}'),
            _ => return None,
        }
    }

    Some(result)
}

fn format_token_callback<'a>(
    lex: &mut Lexer<'a, StringToken<'a>>,
) -> Option<(
    Vec<SpannedToken<Token<'a>>>,
    Vec<SpannedToken<StringFormatToken<'a>>>,
)> {
    let mut expression_lexer = lex.clone().morph::<Token>();
    let mut expression_tokens = Vec::new();
    let mut with_format = false;
    while let Some(token) = expression_lexer.next() {
        match token {
            Ok(Token::SigilCloseCurlyBracket) => break,
            Ok(Token::SigilColon) => {
                with_format = true;
                break;
            }
            Err(_) => return None,
            Ok(tok) => expression_tokens.push(SpannedToken {
                span: expression_lexer.span(),
                token: tok,
            }),
        }
    }

    let mut format_lexer = expression_lexer.morph::<StringFormatToken>();
    let mut format_tokens = Vec::new();
    if with_format {
        while let Some(token) = format_lexer.next() {
            match token {
                Ok(StringFormatToken::FormatEnd) => break,
                Err(_) => return None,
                Ok(tok) => format_tokens.push(SpannedToken {
                    span: format_lexer.span(),
                    token: tok,
                }),
            }
        }
    }

    *lex = format_lexer.morph();
    Some((expression_tokens, format_tokens))
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum StringFormatToken<'a> {
    #[token("}")]
    FormatEnd,
    #[regex(r#"\.[\d]+"#)]
    FormatPrecision(&'a str),
    #[regex(r#"0[\d]+"#)]
    FormatWidth(&'a str),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LexerError {
    #[default]
    NoValidToken,
}

pub fn lex<'a>(input: &'a str) -> Result<Vec<SpannedToken<Token<'a>>>, SpannedToken<LexerError>> {
    Lexer::<Token>::new(input)
        .spanned()
        .map(|(token, span)| match token {
            Ok(token) => Ok(SpannedToken { span, token: token }),
            Err(error) => Err(SpannedToken { span, token: error }),
        })
        .collect()
}
