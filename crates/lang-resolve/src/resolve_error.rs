// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use miette::Diagnostic;
use thiserror::Error;

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {
    /// Symbol not found.
    #[error("Symbol {0} not found while resolving.")]
    SymbolNotFound(ir::QualifiedName),

    /// Symbol not found (retry to load from external).
    #[error("Symbol {0} must be loaded from {1}")]
    SymbolMustBeLoaded(ir::QualifiedName, std::path::PathBuf),

    /// Ambiguous symbol was found
    #[error("Symbol {0} already defined")]
    SymbolAlreadyDefined(ir::QualifiedName),

    /// Ambiguous symbol was found
    #[error("Ambiguous symbol found: {0}")]
    AmbiguousSymbol(ir::QualifiedName, ir::QualifiedNames),

    /// Ambiguous symbol was found
    #[error("Ambiguous identifier '{ambiguous}'")]
    #[allow(missing_docs)]
    AmbiguousId {
        #[label(primary, "First usage of '{first}'")]
        first: ir::Identifier,
        #[label("Ambiguous usage of '{ambiguous}'")]
        ambiguous: ir::Identifier,
    },

    /// Invalid path.
    #[error("Invalid path: {0:?}")]
    InvalidPath(std::path::PathBuf),

    /// Symbol is private
    #[error("Symbol {0} is private")]
    SymbolIsPrivate(ir::QualifiedName),
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
