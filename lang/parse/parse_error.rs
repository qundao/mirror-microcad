// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parser errors

use std::iter::once;
use miette::{Diagnostic, LabeledSpan, SourceCode};
use crate::{parse::*, ty::*};
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error)]
pub enum ParseError {
    /// Error parsing floating point literal
    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(Refer<std::num::ParseFloatError>),

    /// Error parsing integer literal
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(Refer<std::num::ParseIntError>),

    /// Parser rule not found error
    #[error("Rule not found: {0:?}")]
    RuleNotFoundError(Box<crate::parser::Rule>),

    /// IO Error
    #[error("IO Error: {0}")]
    IoError(Refer<std::io::Error>),

    /// Error in pest parser
    #[error("Parser error: {}", .0.variant.message())]
    Parser(Box<pest::error::Error<crate::parser::Rule>>),

    /// Error parsing color literal
    #[error("Error parsing color: {0}")]
    ParseColorError(Refer<microcad_core::ParseColorError>),

    /// Unknown color name
    #[error("Unknown color: {0}")]
    UnknownColorName(Refer<String>),

    /// Unknown unit
    #[error("Unknown unit: {0}")]
    UnknownUnit(Refer<String>),

    /// Unexpected token
    #[error("Unexpected token")]
    UnexpectedToken(SrcRef),

    /// Tuple expression contains both named and positional arguments
    #[error("Tuple expression contains both named and positional arguments")]
    MixedTupleArguments(SrcRef),

    /// Duplicate named argument
    #[error("Duplicate named argument: {0}")]
    DuplicateNamedArgument(Identifier),

    /// Positional argument after named argument
    #[error("Positional argument after named argument")]
    PositionalArgumentAfterNamed(SrcRef),

    /// Empty tuple expression
    #[error("Empty tuple expression")]
    EmptyTupleExpression(SrcRef),

    /// Missing type or value for definition parameter
    #[error("Missing type or value for definition parameter: {0}")]
    ParameterMissingTypeOrValue(Identifier),

    /// Duplicate parameter
    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),

    /// Duplicate argument
    #[error("Duplicate argument: {0}")]
    DuplicateArgument(Identifier),

    /// Duplicated type name in map
    #[error("Duplicated type name in map: {0}")]
    DuplicatedMapType(Identifier),

    /// Duplicate id
    #[error("Duplicate id: {0}")]
    DuplicateIdentifier(Identifier),

    /// Duplicate id in tuple
    #[error("Duplicate id in tuple: {0}")]
    DuplicateTupleIdentifier(Identifier),

    /// Duplicate unnamed type in tuple
    #[error("Duplicate unnamed type in tuple: {0}")]
    DuplicateTupleType(Refer<Type>),

    /// Missing format expression
    #[error("Missing format expression")]
    MissingFormatExpression(SrcRef),

    /// Statement between two init statements
    #[error("Statement between two init statements")]
    StatementBetweenInit(SrcRef),

    /// Loading of a source file failed
    #[error("Loading of source file {1:?} failed: {2}")]
    LoadSource(SrcRef, std::path::PathBuf, std::io::Error),

    /// Grammar rule error
    #[error("Grammar rule error {0}")]
    GrammarRuleError(Refer<String>),

    /// Grammar rule error
    #[error("Invalid qualified name '{0}'")]
    InvalidQualifiedName(Refer<String>),

    /// Grammar rule error
    #[error("Invalid id '{0}'")]
    InvalidIdentifier(Refer<String>),

    /// Qualified name cannot be converted into an Id
    #[error("Qualified name {0} cannot be converted into an Id")]
    QualifiedNameIsNoId(QualifiedName),

    /// Element is not available
    #[error("Element is not available")]
    NotAvailable(SrcRef),

    /// Unknown type
    #[error("Unknown type: {0}")]
    UnknownType(Refer<String>),
}

/// Result with parse error
pub type ParseResult<T> = Result<T, ParseError>;

impl SrcReferrer for ParseError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ParseError::Parser(error) => SrcRef::new(
                match error.location {
                    pest::error::InputLocation::Pos(pos) => std::ops::Range {
                        start: pos,
                        end: pos,
                    },
                    pest::error::InputLocation::Span((start, end)) => {
                        std::ops::Range { start, end }
                    }
                },
                match error.line_col {
                    pest::error::LineColLocation::Pos(pos) => pos.0,
                    pest::error::LineColLocation::Span(start, _) => start.0,
                },
                match error.line_col {
                    pest::error::LineColLocation::Pos(pos) => pos.1,
                    pest::error::LineColLocation::Span(start, _) => start.1,
                },
                0,
            ),
            ParseError::DuplicateNamedArgument(id)
            | ParseError::ParameterMissingTypeOrValue(id)
            | ParseError::DuplicateParameter(id)
            | ParseError::DuplicateArgument(id)
            | ParseError::DuplicatedMapType(id)
            | ParseError::DuplicateIdentifier(id)
            | ParseError::DuplicateTupleIdentifier(id) => id.src_ref(),
            ParseError::QualifiedNameIsNoId(name) => name.src_ref(),
            ParseError::UnexpectedToken(src_ref)
            | ParseError::MixedTupleArguments(src_ref)
            | ParseError::PositionalArgumentAfterNamed(src_ref)
            | ParseError::EmptyTupleExpression(src_ref)
            | ParseError::MissingFormatExpression(src_ref)
            | ParseError::StatementBetweenInit(src_ref)
            | ParseError::NotAvailable(src_ref)
            | ParseError::LoadSource(src_ref , ..) => src_ref.clone(),
            ParseError::ParseFloatError(parse_float_error) => parse_float_error.src_ref(),
            ParseError::ParseIntError(parse_int_error) => parse_int_error.src_ref(),
            ParseError::RuleNotFoundError(_) => SrcRef(None),
            ParseError::IoError(error) => error.src_ref(),
            ParseError::ParseColorError(parse_color_error) => parse_color_error.src_ref(),
            ParseError::UnknownColorName(name) => name.src_ref(),
            ParseError::UnknownUnit(unit) => unit.src_ref(),
            ParseError::DuplicateTupleType(ty) => ty.src_ref(),
            ParseError::GrammarRuleError(rule) => rule.src_ref(),
            ParseError::InvalidQualifiedName(name) => name.src_ref(),
            ParseError::InvalidIdentifier(id) => id.src_ref(),
            ParseError::UnknownType(ty) => ty.src_ref(),
        }
    }
}

impl ParseError {
    /// Add source code to the error
    pub fn with_source(self, source: String) -> ParseErrorWithSource {
        ParseErrorWithSource {
            error: self,
            source_code: Some(source),
        }
    }
}

impl Diagnostic for ParseError {
    fn labels(&self) -> Option<Box<dyn Iterator<Item=LabeledSpan> + '_>> {
        let src_ref = self.src_ref().0?;
        let message = match self {
            ParseError::Parser(err) => {
                err.variant.message().to_string()
            }
            _ => self.to_string()
        };
        let label = LabeledSpan::new(
            Some(message),
            src_ref.range.start,
            src_ref.range.len(),
        );
        Some(Box::new(once(label)))
    }
}

/// Parse error, possibly with source code
#[derive(Debug, Error)]
#[error("{error}")]
pub struct ParseErrorWithSource {
    error: ParseError,
    source_code: Option<String>,
}

impl From<ParseError> for ParseErrorWithSource {
    fn from(value: ParseError) -> Self {
        ParseErrorWithSource {
            error: value,
            source_code: None,
        }
    }
}

impl Diagnostic for ParseErrorWithSource {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source_code.as_ref().map(|source| &*source as &dyn SourceCode)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item=LabeledSpan> + '_>> {
        self.error.labels()
    }
}

impl SrcReferrer for ParseErrorWithSource {
    fn src_ref(&self) -> SrcRef {
        self.error.src_ref()
    }
}