// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code parsing
//!
//! A source file on disc is just a bunch of UTF-8 encoded text which must be parsed
//! before any processing:
//!
//! ```no_run
//! use microcad_lang::lower::ir;
//!
//! let source_file = ir::SourceFile::load("my.µcad").expect("parsing success");
//! ```
//!
//! To read a source file from an already loaded string use:
//!
//! ```no_run
//! use microcad_lang::lower::ir;
//!
//! let source_file = ir::SourceFile::load_from_str(Some("test"), "test.µcad", r#"std::print("hello world!");"#).expect("parsing success");
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be resolved and evaluated (see [`crate::resolve`] and [`crate::eval`])  .

mod attribute;
mod body;
mod call;
mod doc_block;
mod expression;
mod format_string;
mod function;
mod identifier;
mod init_definition;
mod lang_type;
mod literal;
mod module;
mod parameter;
mod source_file;
mod statement;
mod r#type;
mod r#use;
mod workbench;

use microcad_lang_base::{Hashed, Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_parse::ast::LiteralErrorKind;
use microcad_lang_parse::parse;
use miette::{Diagnostic, SourceCode};
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum LowerError {
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[label("{0}")] Refer<std::num::ParseIntError>),

    #[error("Unknown unit: {0}")]
    UnknownUnit(#[label("Unknown unit")] Refer<String>),

    #[error("Duplicate argument: {id}")]
    DuplicateArgument {
        #[label(primary, "Duplicate argument")]
        id: Identifier,
        #[label("Previous declaration")]
        previous: Identifier,
    },

    #[error("Loading of source file {1:?} failed: {2}")]
    LoadSource(SrcRef, std::path::PathBuf, std::io::Error),

    /// Grammar rule error
    #[error("Invalid id '{0}'")]
    InvalidIdentifier(Refer<String>),

    #[error("Unknown type: {0}")]
    UnknownType(#[label("Unknown type")] Refer<String>),

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
    AstParser(Refer<microcad_lang_parse::ParseError>),

    /// An invalid literal was encountered
    #[error("Invalid literal: {error}")]
    InvalidLiteral {
        error: LiteralErrorKind,
        #[label("{error}")]
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
pub type LowerResult<T> = Result<T, LowerError>;

impl SrcReferrer for LowerError {
    fn src_ref(&self) -> SrcRef {
        match self {
            LowerError::DuplicateArgument { id, .. } => id.src_ref(),
            LowerError::LoadSource(src_ref, ..)
            | LowerError::InvalidGlobPattern(src_ref)
            | LowerError::UseGlobAlias(src_ref)
            | LowerError::InvalidLiteral { src_ref, .. }
            | LowerError::InvalidExpression { src_ref }
            | LowerError::InvalidStatement { src_ref }
            | LowerError::InvalidRangeType { src_ref } => src_ref.clone(),
            LowerError::ParseIntError(parse_int_error) => parse_int_error.src_ref(),
            LowerError::InvalidIdentifier(id) => id.src_ref(),
            LowerError::UnknownUnit(unit) => unit.src_ref(),
            LowerError::UnknownType(ty) => ty.src_ref(),
            LowerError::InvalidMatrixType(ty) => ty.src_ref(),
            LowerError::AstParser(err) => err.src_ref(),
        }
    }
}

/// Parse error, possibly with source code
#[derive(Debug, Error)]
#[error("Failed to parse")] // todo
pub struct LowerErrorsWithSource {
    /// The errors encountered during parsing
    pub errors: Vec<LowerError>,
    /// The parsed source code
    pub source_code: Option<Hashed<String>>,
}

impl From<LowerError> for LowerErrorsWithSource {
    fn from(value: LowerError) -> Self {
        LowerErrorsWithSource {
            errors: vec![value],
            source_code: None,
        }
    }
}

impl From<Vec<LowerError>> for LowerErrorsWithSource {
    fn from(value: Vec<LowerError>) -> Self {
        LowerErrorsWithSource {
            errors: value,
            source_code: None,
        }
    }
}

impl Diagnostic for LowerErrorsWithSource {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source_code
            .as_ref()
            .map(|source| source.value() as &dyn SourceCode)
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        Some(Box::new(
            self.errors.iter().map(|e| -> &dyn Diagnostic { e }),
        ))
    }
}

impl SrcReferrer for LowerErrorsWithSource {
    fn src_ref(&self) -> SrcRef {
        self.errors[0].src_ref()
    }
}

pub(crate) fn build_ast(
    source: &str,
    lower_context: &super::LowerContext,
) -> Result<microcad_lang_parse::ast::Program, LowerErrorsWithSource> {
    parse(source).map_err(|errors| {
        let errors = errors
            .into_iter()
            .map(|error| {
                let src_ref = lower_context.src_ref(&error.span);
                LowerError::AstParser(Refer::new(error, src_ref))
            })
            .collect::<Vec<_>>();
        LowerErrorsWithSource {
            errors,
            source_code: Some(
                lower_context
                    .source
                    .clone()
                    .map(|source| source.to_string()),
            ),
        }
    })
}
