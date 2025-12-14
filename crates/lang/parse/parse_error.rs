// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parser errors

use miette::{Diagnostic, LabeledSpan, SourceCode};
use crate::{parse::*, ty::*};
use microcad_syntax::ast::LiteralErrorKind;
use std::iter::once;
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum ParseError {
    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(
        #[label("Error parsing floating point literal: {0}")]
        Refer<std::num::ParseFloatError>
    ),

    #[error("Error parsing integer literal: {0}")]
    ParseIntError(
        #[label("{0}")]
        Refer<std::num::ParseIntError>
    ),

    /// Error parsing color literal
    #[error("Error parsing color: {0}")]
    ParseColorError(
        #[label("Invalid color literal")]
        Refer<microcad_core::ParseColorError>
    ),

    #[error("Unknown color: {0}")]
    UnknownColorName(
        #[label("Unknown color")]
        Refer<String>
    ),

    #[error("Unknown unit: {0}")]
    UnknownUnit(
        #[label("Unknown unit")]
        Refer<String>
    ),

    #[error("Unexpected token")]
    UnexpectedToken(
        #[label("Unexpected token")]
        SrcRef
    ),

    #[error("Tuple expression contains both named and positional arguments")]
    MixedTupleArguments(
        #[label("Mixed tuple expression")]
        SrcRef
    ),

    #[error("Duplicate named argument: {0}")]
    DuplicateNamedArgument(
        #[label("Duplicate argument")]
        Identifier
    ),

    #[error("Positional argument after named argument")]
    PositionalArgumentAfterNamed(
        #[label("Positional argument after named argument")]
        SrcRef
    ),

    #[error("Empty tuple expression")]
    EmptyTupleExpression(
        #[label("Empty expression")]
        SrcRef
    ),

    #[error("Missing type or value for definition parameter: {0}")]
    ParameterMissingTypeOrValue(
        #[label("Missing type or value")]
        Identifier
    ),

    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(
        #[label("Duplicate parameter")]
        Identifier
    ),

    #[error("Duplicate argument: {0}")]
    DuplicateArgument(
        #[label("Duplicate argument")]
        Identifier
    ),

    #[error("Duplicated type name in map: {0}")]
    DuplicatedMapType(
        #[label("Duplicate type")]
        Identifier
    ),

    #[error("Duplicate id: {0}")]
    DuplicateIdentifier(
        #[label("Duplicate identifier")]
        Identifier
    ),

    #[error("Duplicate id in tuple: {id}")]
    DuplicateTupleIdentifier {
        #[label(primary, "Duplicate identifier")]
        id: Identifier,
        #[label("Previous declaration")]
        previous: Identifier,
    },

    #[error("Duplicate unnamed type in tuple: {0}")]
    DuplicateTupleType(
        #[label("Duplicate item")]
        Refer<Type>
    ),

    #[error("Missing format expression")]
    MissingFormatExpression(
        #[label("Missing expression")]
        SrcRef
    ),

    #[error("Statement between two init statements")]
    StatementBetweenInit(
        #[label("Statement between two init statements")]
        SrcRef
    ),

    #[error("Loading of source file {1:?} failed: {2}")]
    LoadSource(SrcRef, std::path::PathBuf, std::io::Error),

    #[error("Grammar rule error {0}")]
    GrammarRuleError(
        #[label("Invalid grammar rule")]
        Refer<String>
    ),

    #[error("Invalid qualified name '{0}'")]
    InvalidQualifiedName(
        #[label("Invalid name")]
        Refer<String>
    ),

    #[error("Invalid id '{0}'")]
    InvalidIdentifier(
        #[label("Invalid identifier")]
        Refer<String>
    ),

    #[error("Qualified name {0} cannot be converted into an Id")]
    QualifiedNameIsNoId(
        #[label("Invalid name")]
        QualifiedName
    ),

    #[error("Element is not available")]
    NotAvailable(
        #[label("Element is not available")]
        SrcRef
    ),

    #[error("Unknown type: {0}")]
    UnknownType(
        #[label("Unknown type")]
        Refer<String>
    ),

    #[error("If expression must return a value in all cases")]
    IncompleteIfExpression(
        #[label("Incomplete if expression")]
        SrcRef
    ),

    /// Matrix type with invalid dimensions
    #[error("Invalid matrix type: {0}")]
    InvalidMatrixType(Refer<String>),

    /// Invalid glob pattern
    #[error("Invalid glob pattern, wildcard must be at the end of the pattern")]
    InvalidGlobPattern(SrcRef),

    /// A glob import is given an alias
    #[error("Glob imports can't be given an alias")]
    UseGlobAlias(SrcRef),

    /// A parser from the AST builder
    #[error("{error}")]
    AstParser {
        src_ref: SrcRef,
        error: microcad_syntax::ParseError,
    },

    /// Call attribute with a non-identifier name
    #[error("Call attributes must have a plain identifier as name")]
    InvalidAttributeCall(QualifiedName),

    /// An invalid literal was encountered
    #[error("Invalid literal: {error}")]
    InvalidLiteral {
        error: LiteralErrorKind,
        src_ref: SrcRef,
    },

    /// An invalid expression was encountered
    #[error("Invalid expression")]
    InvalidExpression { src_ref: SrcRef },

    /// An invalid state was encountered
    #[error("Invalid statement")]
    InvalidStatement { src_ref: SrcRef },

    /// A type range between non-integer literals
    #[error("range expressions must be between integers")]
    InvalidRangeType { src_ref: SrcRef },
}

/// Result with parse error
pub type ParseResult<T> = Result<T, ParseError>;

impl SrcReferrer for ParseError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ParseError::DuplicateNamedArgument(id)
            | ParseError::ParameterMissingTypeOrValue(id)
            | ParseError::DuplicateParameter(id)
            | ParseError::DuplicateArgument(id)
            | ParseError::DuplicatedMapType(id)
            | ParseError::DuplicateIdentifier(id)
            | ParseError::DuplicateTupleIdentifier{id, ..} => id.src_ref(),
            ParseError::QualifiedNameIsNoId(name) | ParseError::InvalidAttributeCall(name) => {
                name.src_ref()
            }
            ParseError::UnexpectedToken(src_ref)
            | ParseError::MixedTupleArguments(src_ref)
            | ParseError::PositionalArgumentAfterNamed(src_ref)
            | ParseError::EmptyTupleExpression(src_ref)
            | ParseError::MissingFormatExpression(src_ref)
            | ParseError::StatementBetweenInit(src_ref)
            | ParseError::NotAvailable(src_ref)
            | ParseError::IncompleteIfExpression(src_ref)
            | ParseError::LoadSource(src_ref, ..)
            | ParseError::InvalidGlobPattern(src_ref)
            | ParseError::UseGlobAlias(src_ref)
            | ParseError::AstParser { src_ref, .. }
            | ParseError::InvalidLiteral { src_ref, .. }
            | ParseError::InvalidExpression { src_ref }
            | ParseError::InvalidStatement { src_ref }
            | ParseError::InvalidRangeType { src_ref } => src_ref.clone(),
            ParseError::ParseFloatError(parse_float_error) => parse_float_error.src_ref(),
            ParseError::ParseIntError(parse_int_error) => parse_int_error.src_ref(),
            ParseError::ParseColorError(parse_color_error) => parse_color_error.src_ref(),
            ParseError::UnknownColorName(name) => name.src_ref(),
            ParseError::UnknownUnit(unit) => unit.src_ref(),
            ParseError::DuplicateTupleType(ty) => ty.src_ref(),
            ParseError::GrammarRuleError(rule) => rule.src_ref(),
            ParseError::InvalidQualifiedName(name) => name.src_ref(),
            ParseError::InvalidIdentifier(id) => id.src_ref(),
            ParseError::UnknownType(ty) => ty.src_ref(),
            ParseError::InvalidMatrixType(ty) => ty.src_ref(),
        }
    }
}

impl ParseError {
    /// Add source code to the error
    pub fn with_source(self, source: String) -> ParseErrorsWithSource {
        ParseErrorsWithSource {
            errors: vec![self],
            source_code: Some(source),
            source_hash: 0,
        }
    }
}

/// Parse error, possibly with source code
#[derive(Debug, Error)]
#[error("Failed to parse")] // todo
pub struct ParseErrorsWithSource {
    /// The errors encountered during parsing
    pub errors: Vec<ParseError>,
    /// The parsed source code
    pub source_code: Option<String>,
    /// The hash of the parsed source
    pub source_hash: u64,
}

impl From<ParseError> for ParseErrorsWithSource {
    fn from(value: ParseError) -> Self {
        ParseErrorsWithSource {
            errors: vec![value],
            source_code: None,
            source_hash: 0,
        }
    }
}

impl From<Vec<ParseError>> for ParseErrorsWithSource {
    fn from(value: Vec<ParseError>) -> Self {
        ParseErrorsWithSource {
            errors: value,
            source_code: None,
            source_hash: 0,
        }
    }
}

impl Diagnostic for ParseErrorsWithSource {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source_code
            .as_ref()
            .map(|source| source as &dyn SourceCode)
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        Some(Box::new(
            self.errors.iter().map(|e| -> &dyn Diagnostic { e }),
        ))
    }
}

impl SrcReferrer for ParseErrorsWithSource {
    fn src_ref(&self) -> SrcRef {
        self.errors[0].src_ref()
    }
}
