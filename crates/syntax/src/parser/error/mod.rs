// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod rich;

use crate::Span;
use crate::tokens::Token;
use miette::{Diagnostic, LabeledSpan};
pub use rich::{Rich, RichReason};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::once;
use thiserror::Error;

/// An error from building the abstract syntax tree
#[derive(Debug)]
pub struct ParseError {
    /// The span of the source that caused the error
    pub span: Span,
    error: Rich<'static, Token<'static>, Span, ParseErrorKind>,
}

impl ParseError {
    pub(crate) fn new<'tokens>(error: Rich<'tokens, Token<'tokens>, Span, ParseErrorKind>) -> Self {
        Self {
            span: error.span().clone(),
            error: error.map_token(Token::into_owned).into_owned(),
        }
    }
}

#[derive(Debug, Error, Clone, Diagnostic)]
pub enum ParseErrorKind {
    #[error("{0} is a reserved keyword")]
    ReservedAttribute(&'static str),
    #[error("{0} is a reserved keyword and can't be used as an identifier")]
    ReservedAttributeAsIdentifier(&'static str),
    #[error("unclosed string")]
    UnterminatedString,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.error.reason() {
            RichReason::Custom(error) => write!(f, "{error}"),
            RichReason::ExpectedFound { expected, .. } => {
                write!(f, "Expected ")?;
                let mut expected = expected.iter();
                if let Some(pattern) = expected.next() {
                    write!(f, "{pattern}")?;
                }
                let last = expected.next_back();
                for pattern in expected {
                    write!(f, ", {pattern}")?;
                }
                if let Some(pattern) = last {
                    write!(f, " or {pattern}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for ParseError {}

impl Diagnostic for ParseError {
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self.error.reason() {
            RichReason::Custom(error) => error.help(),
            _ => None
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let msg = match self.error.reason() {
            RichReason::Custom(error) => {
                if let Some(labels) = error.labels() {
                    return Some(labels);
                }
                error.to_string()
            },
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
