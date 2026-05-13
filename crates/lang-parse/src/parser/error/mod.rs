// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod rich;

use crate::ParseContext;
use crate::parser::error::rich::RichPattern;
use crate::tokens::Token;
use microcad_lang_base::{Diagnostics, Refer, Span};
use miette::{Diagnostic, LabeledSpan};
pub use rich::{Rich, RichReason};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::once;
use thiserror::Error;

/// Type alias for RichError
pub type RichError<'tokens> = Rich<'tokens, Token<'tokens>, Span, ParseErrorKind>;

/// An error from building the abstract syntax tree
#[derive(Debug)]
pub struct ParseError {
    /// The span of the source that caused the error
    pub span: Span,
    error: RichError<'static>,
}

impl ParseError {
    pub(crate) fn new<'tokens>(error: RichError<'tokens>) -> Self {
        Self {
            span: error.span().clone(),
            error: error.map_token(Token::into_owned).into_owned(),
        }
    }
}

/// Parse error collection.
#[derive(Debug, Error, derive_more::Deref, miette::Diagnostic)]
pub struct ParseErrors(#[related] pub Vec<ParseError>);

impl ParseErrors {
    /// Convert parse errors to diagnostics
    pub fn to_diagnostics(self, context: &ParseContext) -> Diagnostics {
        let mut diag_list = Diagnostics::default();
        use microcad_lang_base::{Diagnostic as D, PushDiag};

        for err in self.0 {
            let span = err.span.clone();
            diag_list
                .push_diag(D::Error(Refer::new(err.into(), context.src_ref(&span))))
                .expect("Diag list should return no error");
        }

        diag_list
    }
}

impl std::fmt::Display for ParseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Found {} parse errors", self.0.len())
    }
}

impl<'tokens> From<Vec<RichError<'tokens>>> for ParseErrors {
    fn from(errors: Vec<RichError<'tokens>>) -> Self {
        Self(errors.into_iter().map(ParseError::new).collect())
    }
}

#[derive(Debug, Error, Clone, Diagnostic)]
pub enum ParseErrorKind {
    #[error("'{0}' is a reserved keyword")]
    ReservedKeyword(&'static str),
    #[error("'{0}' is a reserved keyword and can't be used as an identifier")]
    ReservedKeywordAsIdentifier(&'static str),
    #[error("'{0}' is a keyword and can't be used as an identifier")]
    KeywordAsIdentifier(&'static str),
    #[error("unclosed string")]
    UnterminatedString,
    #[error("Unclosed {kind}")]
    UnclosedBracket {
        #[label("{kind} opened here")]
        open: Span,
        #[label("expected {kind} to be closed by here with a '{close_token}'")]
        end: Span,
        kind: &'static str,
        close_token: Token<'static>,
    },
    #[error("Expression statements need to have a trailing semicolon")]
    ExpressionMissingSemicolon {
        #[label("exprected a semicolon after this expression")]
        span: Span,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.error.reason() {
            RichReason::Custom(error) => write!(f, "{error}"),
            RichReason::ExpectedFound { expected, .. } => {
                write!(f, "Expected ")?;
                let mut expected = expected.iter().filter(|pat| match pat {
                    // don't show 'whitespace' as possible tokens, if there are also others
                    RichPattern::Label(label) if expected.len() > 1 => label != "whitespace",
                    _ => true,
                });
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
            _ => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let msg = match self.error.reason() {
            RichReason::Custom(error) => {
                if let Some(labels) = error.labels() {
                    return Some(labels);
                }
                error.to_string()
            }
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
