// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parser errors

use miette::{Diagnostic, SourceCode};
use crate::{parse::*, ty::*};
use microcad_syntax::ast::LiteralErrorKind;
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum ParseError {
    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(
        #[label("{0}")]
        Refer<std::num::ParseFloatError>
    ),

    #[error("Error parsing integer literal: {0}")]
    ParseIntError(
        #[label("{0}")]
        Refer<std::num::ParseIntError>
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

    #[error("Missing type or value for definition parameter: {0}")]
    ParameterMissingTypeOrValue(
        #[label("Missing type or value")]
        Identifier
    ),

    #[error("Duplicate argument: {id}")]
    DuplicateArgument {
        #[label(primary, "Duplicate argument")]
        id: Identifier,
        #[label("Previous declaration")]
        previous: Identifier,
    },

    #[error("Duplicate id: {id}")]
    DuplicateIdentifier {
        #[label(primary, "Duplicate identifier")]
        id: Identifier,
        #[label("Previous declaration")]
        previous: Identifier,
    },

    #[error("Duplicate id in tuple: {id}")]
    DuplicateTupleIdentifier {
        #[label(primary, "Duplicate identifier")]
        id: Identifier,
        #[label("Previous declaration")]
        previous: Identifier,
    },

    #[error("Duplicate unnamed type in tuple: {ty}")]
    DuplicateTupleType {
        #[label(primary, "Duplicate item")]
        ty: Refer<Type>,
        #[label("Previous declaration")]
        previous: Refer<Type>,
    },

    #[error("Loading of source file {1:?} failed: {2}")]
    LoadSource(SrcRef, std::path::PathBuf, std::io::Error),

    /// Grammar rule error
    #[error("Invalid id '{0}'")]
    InvalidIdentifier(Refer<String>),

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
    #[error(transparent)]
    #[diagnostic(transparent)]
    AstParser(Refer<microcad_syntax::ParseError>),

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
            ParseError::ParameterMissingTypeOrValue(id)
            | ParseError::DuplicateArgument{id, ..}
            | ParseError::DuplicateIdentifier{id, ..}
            | ParseError::DuplicateTupleIdentifier{id, ..} => id.src_ref(),
            ParseError::UnexpectedToken(src_ref)
            | ParseError::NotAvailable(src_ref)
            | ParseError::IncompleteIfExpression(src_ref)
            | ParseError::LoadSource(src_ref, ..)
            | ParseError::InvalidGlobPattern(src_ref)
            | ParseError::UseGlobAlias(src_ref)
            | ParseError::InvalidLiteral { src_ref, .. }
            | ParseError::InvalidExpression { src_ref }
            | ParseError::InvalidStatement { src_ref }
            | ParseError::InvalidRangeType { src_ref } => src_ref.clone(),
            ParseError::ParseFloatError(parse_float_error) => parse_float_error.src_ref(),
            ParseError::ParseIntError(parse_int_error) => parse_int_error.src_ref(),
            ParseError::InvalidIdentifier(id) => id.src_ref(),
            ParseError::UnknownUnit(unit) => unit.src_ref(),
            ParseError::DuplicateTupleType{ty, ..} => ty.src_ref(),
            ParseError::UnknownType(ty) => ty.src_ref(),
            ParseError::InvalidMatrixType(ty) => ty.src_ref(),
            ParseError::AstParser(err) => err.src_ref(),
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
