use logos::{Lexer, Logos, Span};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexerError))]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r#"\/\/[^\n]*"#, allow_greedy = true)]
    SingleLineComment,
    #[regex(r#"(?m)/\*(.|\n)+?\*/"#)]
    MultiLineComment,
    #[regex(r#"\/\/\/[^\n]*"#, allow_greedy = true)]
    DocComment,

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
    Identifier,

    #[regex(r#"-?(0|[1-9]\d*)"#)]
    LiteralInt,
    #[regex(r#"-?(0|[1-9]\d*)?\.(\d+)((e|E)(-|\+)?(\d+))?"#)]
    LiteralFloat,
    #[token(r#"""#, string_token_callback)]
    LiteralString(Vec<Spanned<StringToken>>),
    #[regex("true|false")]
    LiteralBool,

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
    OperatorInterset,
    #[token("^")]
    OperatorXor,
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
    #[token("!")]
    OperatorNot,
    #[token("=")]
    OperatorAssignment,
}

fn string_token_callback(lex: &mut Lexer<Token>) -> Option<Vec<Spanned<StringToken>>> {
    let mut string_lexer = lex.clone().morph::<StringToken>();
    let mut tokens = Vec::new();
    while let Some(token) = string_lexer.next() {
        match token {
            Ok(StringToken::Quote) => break,
            Err(_) => return None,
            Ok(tok) => tokens.push(Spanned {
                span: string_lexer.span(),
                val: tok
            }),
        }
    }
    *lex = string_lexer.morph();
    Some(tokens)
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum StringToken {
    #[regex(r#"[^"{}\\]+"#)]
    Content,
    #[regex(r#"\\["\\/bfnrt]"#)]
    Escaped,
    #[token(r#"\"#)]
    BackSlash,
    #[token(r#"{{"#)]
    EscapedCurlyOpen,
    #[token(r#"}}"#)]
    EscapedCurlyClose,
    #[token("{", format_token_callback)]
    FormatStart((Vec<Spanned<Token>>, Vec<Spanned<StringFormatToken>>)),
    #[token(r#"""#)]
    Quote,
}

fn format_token_callback(lex: &mut Lexer<StringToken>) -> Option<(Vec<Spanned<Token>>, Vec<Spanned<StringFormatToken>>)> {
    let mut expression_lexer = lex.clone().morph::<Token>();
    let mut expression_tokens = Vec::new();
    let mut with_format = false;
    while let Some(token) = expression_lexer.next() {
        match token {
            Ok(Token::SigilCloseCurlyBracket) => break,
            Ok(Token::SigilColon) => {
                with_format = true;
                break
            },
            Err(_) => return None,
            Ok(tok) => expression_tokens.push(Spanned {
                span: expression_lexer.span(),
                val: tok,
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
                Ok(tok) => format_tokens.push(Spanned {
                    span: format_lexer.span(),
                    val: tok,
                }),
            }
        }
    }

    *lex = format_lexer.morph();
    Some((expression_tokens, format_tokens))
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum StringFormatToken {
    #[token("}")]
    FormatEnd,
    #[regex(r#"\.[\d]+"#)]
    FormatPrecision,
    #[regex(r#"0[\d]+"#)]
    FormatWidth,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub val: T,
}

impl<T: PartialEq> PartialEq<T> for Spanned<T> {
    fn eq(&self, other: &T) -> bool {
        self.val.eq(other)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LexerError {
    #[default]
    NoValidToken,
}

pub fn lex(input: &str) -> Result<Vec<Spanned<Token>>, Spanned<LexerError>> {
    Lexer::<Token>::new(input)
        .spanned()
        .map(|(token, span)| match token {
            Ok(token) => {
                dbg!(&input[span.start..span.end]);
                Ok(Spanned { span, val: token })
            },
            Err(error) => Err(Spanned { span, val: error }),
        })
        .collect()
}
