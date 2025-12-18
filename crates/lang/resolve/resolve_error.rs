// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use miette::Diagnostic;
use thiserror::Error;

use crate::{diag::*, parse::*, syntax::*};
use crate::src_ref::{SrcRef, SrcReferrer};

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {
    /// Parse Error.
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseErrorWithSource),

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
    #[error("{0} is not available within {1}")]
    StatementNotSupported(String, String),

    /// Resolve check failed
    #[error("Resolve failed")]
    ResolveCheckFailed,

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
    SourceFileNotFound(
        #[label("module not found")]
        Identifier,
        std::path::PathBuf
    ),

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

impl SrcReferrer for ResolveError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ResolveError::SourceFileNotFound(identifier, _) => identifier.src_ref(),
            _ => SrcRef(None),
        }
    }
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
