// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use compact_str::ToCompactString;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Name, SingleIdentifier, SrcRef, SrcReferrer, SymbolId};

/// Unresolve symbol name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolName {
    /// Full name of the symbol
    pub full_name: Name,
    /// The original symbol id.
    pub symbol_id: SymbolId,
}

impl SymbolName {
    pub fn invalid(symbol_id: SymbolId) -> Self {
        Self {
            full_name: "<INVALID>".to_compact_string(),
            symbol_id,
        }
    }
}

impl SingleIdentifier for SymbolName {
    fn single_identifier(&self) -> Option<&crate::Identifier> {
        None
    }
}

impl SrcReferrer for SymbolName {
    fn src_ref(&self) -> SrcRef {
        SrcRef::none()
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum UnresolveError {
    #[error("Could not unresolved symbol: {0}")]
    Fail(SymbolId),
}

/// A struct that can resolve symbol IDs into human-readable names.
pub trait Unresolver {
    fn unresolve_id(&self, symbol_id: impl Into<SymbolId>) -> Option<SymbolName>;

    fn unresolve(&mut self, symbol_id: impl Into<SymbolId>) -> SymbolName {
        let id = symbol_id.into();
        match self.unresolve_id(id.clone()) {
            Some(name) => name,
            None => {
                self.error(UnresolveError::Fail(id.clone()));
                SymbolName::invalid(id.clone())
            }
        }
    }

    /// Error handler
    fn error(&mut self, _e: UnresolveError) {
        // Do nothing by default.
    }
}

/// Implemented by types (like IR nodes) that expand internal IDs back into
/// human-readable names for target representation `T`.
pub trait Unresolve<T> {
    fn unresolve_symbols<U: Unresolver>(self, unresolver: &mut U) -> T;
}
