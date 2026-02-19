// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(unused_assignments)]
//! Resolve error

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::resolve::Symbol;
use crate::resolve::grant::Grant;
use crate::src_ref::{SrcRef, SrcReferrer};
use crate::{diag::*, parse::*, syntax::*};

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

    /// Sternal module file not found
    #[error("Ambiguous external module files found {0:?}")]
    AmbiguousExternals(Vec<std::path::PathBuf>),

    /// Ambiguous symbol was found
    #[error("Symbol {0} already defined")]
    SymbolAlreadyDefined(QualifiedName),

    /// Ambiguous symbol was found
    #[error("Ambiguous symbol found: {0}")]
    AmbiguousSymbol(QualifiedName, QualifiedNames),

    /// Ambiguous symbol was found
    #[error("Ambiguous identifier '{ambiguous}'")]
    #[allow(missing_docs)]
    AmbiguousId {
        #[label(primary, "First usage of '{first}'")]
        first: Identifier,
        #[label("Ambiguous usage of '{ambiguous}'")]
        ambiguous: Identifier,
    },

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
    #[allow(missing_docs)]
    CodeBetweenInitializers {
        #[label("Between these initializers")]
        initializers: SrcRef,
        #[label(primary, "This statement is not allowed")]
        statement: SrcRef,
        #[label("Inside this {kind}")]
        workbench: SrcRef,
        kind: &'static str,
    },

    /// Statement not allowed prior initializers
    #[error("Statement not allowed prior initializers")]
    #[allow(missing_docs)]
    StatementNotAllowedPriorInitializers {
        #[label("Before this initializer")]
        initializer: SrcRef,
        #[label(primary, "This statement is not allowed")]
        statement: SrcRef,
        #[label("Inside this {kind}")]
        workbench: SrcRef,
        kind: &'static str,
    },
}

/// Statement is not supported in this context.
#[derive(Debug, Error, Diagnostic)]
#[error("{} is not allowed {} {}", capitalize_first(inner), self.placement(), self.outer())]
#[diagnostic(help("{inner} is only allowed within {}", self.allowed_parents()))]
pub struct StatementNotSupportedError {
    inner: &'static str,
    #[label(primary, "This {inner} is not allowed{}", self.maybe_here())]
    inner_span: SrcRef,
    outer: &'static str,
    #[label("Within this {outer}")]
    outer_span: Option<SourceSpan>,
    allowed_parents: &'static [&'static str],
}

impl StatementNotSupportedError {
    /// Create an error from inner node name, src_ref and parent
    pub(super) fn new<T: Grant + SrcReferrer>(node: &T, parent: &Symbol) -> Self {
        StatementNotSupportedError {
            inner: node.kind(),
            inner_span: node.kind_ref().unwrap_or_else(|| node.src_ref()),
            outer: parent.kind_str(),
            outer_span: (!parent.is_source())
                .then(|| parent.kind_ref().unwrap_or_else(|| parent.src_ref()))
                .and_then(|src_ref| src_ref.as_miette_span()),
            allowed_parents: node.allowed_parents(),
        }
    }

    fn allowed_parents(&self) -> impl std::fmt::Display {
        struct AllowedParents(&'static [&'static str]);

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

    fn parent_is_root(&self) -> bool {
        self.outer == "source file"
    }

    fn placement(&self) -> &'static str {
        if self.parent_is_root() {
            "at"
        } else {
            "within"
        }
    }

    fn outer(&self) -> &'static str {
        if self.parent_is_root() {
            "source root"
        } else {
            self.outer
        }
    }

    fn maybe_here(&self) -> &'static str {
        if self.parent_is_root() { " here" } else { "" }
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
