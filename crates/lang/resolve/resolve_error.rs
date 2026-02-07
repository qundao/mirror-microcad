// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(unused_assignments)]
//! Resolve error

use miette::Diagnostic;
use thiserror::Error;

use crate::src_ref::{SrcRef, SrcReferrer};
use crate::{diag::*, parse::*, syntax::*};
use crate::resolve::grant::Grant;
use crate::resolve::Symbol;

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {
    /// Parse Error.
    #[error("Parse Error: {0}")]
    #[diagnostic(transparent)]
    ParseError(#[from] ParseErrorsWithSource),

    /// Can't find a project file by hash.
    #[error("Could not find a file with hash {0}")]
    UnknownHash(u64),

    /// Hash is zero
    #[error("Hash is zero")]
    NulHash,

    /// Name of external symbol is unknown.
    #[error("External symbol `{0}` not found")]
    ExternalSymbolNotFound(QualifiedName),

    /// Path of external file is unknown.
    #[error("External path `{0}` not found")]
    ExternalPathNotFound(std::path::PathBuf),

    /// Can't find a project file by it's path.
    #[error("Could not find a file with path {0}")]
    FileNotFound(std::path::PathBuf),

    /// Symbol not found.
    #[error("Symbol {0} not found while resolving.")]
    SymbolNotFound(QualifiedName),

    /// Symbol not found (retry to load from external).
    #[error("Symbol {0} must be loaded from {1}")]
    SymbolMustBeLoaded(QualifiedName, std::path::PathBuf),

    /// Symbol is not a value
    #[error("Symbol {0} is not a value")]
    NotAValue(QualifiedName),

    /// Declaration of property not allowed here
    #[error("Declaration of {0} not allowed within {1}")]
    DeclNotAllowed(Identifier, QualifiedName),

    /// Sternal module file not found
    #[error("Ambiguous external module files found {0:?}")]
    AmbiguousExternals(Vec<std::path::PathBuf>),

    /// Ambiguous symbol was found
    #[error("Symbol {0} already defined")]
    SymbolAlreadyDefined(QualifiedName),

    /// Ambiguous symbol was found
    #[error("Ambiguous symbol found: {0}")]
    AmbiguousSymbol(QualifiedName, QualifiedNames),

    /// ScanDir Error
    #[error("{0}")]
    ScanDirError(#[from] scan_dir::Error),

    /// Invalid path.
    #[error("Invalid path: {0:?}")]
    InvalidPath(std::path::PathBuf),

    /// Diagnostic error
    #[error("Diagnostic error: {0}")]
    DiagError(#[from] DiagError),

    /// Statement is not supported in this context.
    #[error(transparent)]
    #[diagnostic(transparent)]
    StatementNotSupported(#[from] StatementNotSupportedError),

    /// Resolve check failed
    #[error("Resolve failed")]
    ResolveCheckFailed(SrcRef),

    /// Symbol is private
    #[error("Symbol {0} is private")]
    SymbolIsPrivate(QualifiedName),

    /// ScanDir Error
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    /// Invalid path.
    #[error(
        "Source of module '{0}' could not be found in {1:?} (expecting a file '{0}.µcad' or '{0}/mod.µcad')"
    )]
    SourceFileNotFound(#[label("module not found")] Identifier, std::path::PathBuf),

    /// Wrong lookup target
    #[error("Wrong lookup target")]
    WrongTarget,

    /// Statement not allowed within workbenches
    #[error("Statement not allowed within workbenches")]
    IllegalWorkbenchStatement,

    /// Code Between initializers
    #[error("Code between initializers is not allowed")]
    CodeBetweenInitializers,

    /// Statement not allowed prior initializers
    #[error("Statement not allowed prior initializers")]
    StatementNotAllowedPriorInitializers,
}


/// Statement is not supported in this context.
#[derive(Debug, Error, Diagnostic)]
#[error("{} is not available within {outer}", capitalize_first(inner))]
#[diagnostic(help("{inner} is only allowed within {}", self.allowed_parents()))]
pub struct StatementNotSupportedError {
    inner: &'static str,
    #[label(primary, "This {inner} is not allowed")]
    inner_span: SrcRef,
    outer: &'static str,
    #[label("Within this {outer}")]
    outer_span: SrcRef,
    allowed_parents: &'static[&'static str],
}

impl StatementNotSupportedError {
    /// Create an error from inner node name, src_ref and parent
    pub(super) fn new<T: Grant + SrcReferrer>(node: &T, parent: &Symbol) -> Self {
        StatementNotSupportedError {
            inner: node.kind(),
            inner_span: node.src_ref(),
            outer: parent.kind_str(),
            outer_span: parent.src_ref(),
            allowed_parents: node.allowed_parents(),
        }
    }

    fn allowed_parents(&self) -> impl std::fmt::Display {
        struct AllowedParents(&'static[&'static str]);

        impl std::fmt::Display for AllowedParents {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut items = self.0.iter();
                if let Some(first) = items.next() {
                    write!(f, "{first}")?;
                }
                let last = items.next_back();
                for item in items {
                    write!(f, ", {item}")?;
                }
                if let Some(last) = last {
                    write!(f, " or {last}")?;
                }
                Ok(())
            }
        }

        AllowedParents(self.allowed_parents)
    }
}

impl SrcReferrer for ResolveError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ResolveError::SourceFileNotFound(identifier, _) => identifier.src_ref(),
            ResolveError::ParseError(parse_error) => parse_error.src_ref(),
            ResolveError::ResolveCheckFailed(src_ref) => src_ref.clone(),
            _ => SrcRef(None),
        }
    }
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
