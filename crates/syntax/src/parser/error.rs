use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::Span;
use crate::tokens::Token;
use chumsky::error::{Rich, RichReason};
use miette::{Diagnostic, LabeledSpan};
use std::iter::once;

#[derive(Debug)]
pub struct ParseError {
    pub span: Span,
    error: Rich<'static, Token<'static>, Span>,
}

impl ParseError {
    pub fn new<'tokens>(error: Rich<'tokens, Token<'tokens>, Span>) -> Self {
        Self {
            span: error.span().clone(),
            error: error.map_token(Token::into_owned).into_owned(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.error.reason() {
            RichReason::Custom(error) => write!(f, "{error}"),
            RichReason::ExpectedFound {
                expected, ..
            } => {
                write!(f, "Expected ")?;
                let mut expected = expected.iter();
                if let Some(pattern) = expected.next() {
                    write!(f, "{pattern}")?;
                }
                let last = expected.next_back();
                for pattern in expected {
                    write!(f, ", {pattern}")?;
                };
                if let Some(pattern) = last {
                    write!(f, " or {pattern}")?;
                }
                Ok(())
            },
        }
    }
}

impl Error for ParseError {

}

impl Diagnostic for ParseError {
    fn labels(&self) -> Option<Box<dyn Iterator<Item=LabeledSpan> + '_>> {
        let msg = match self.error.reason() {
            RichReason::Custom(error) => error.clone(),
            RichReason::ExpectedFound {
                found: Some(found), ..
            } if found.is_error() => found.kind().into(),
            RichReason::ExpectedFound {
                found: Some(found), ..
            } => format!("unexpected {}", found.kind()),
            RichReason::ExpectedFound { found: None, .. } => "unexpected token".into(),
        };
        Some(Box::new(once(LabeledSpan::new(
            Some(msg),
            self.span.start,
            self.span.len(),
        ))))
    }
}
