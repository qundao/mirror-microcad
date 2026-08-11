// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Name, SymbolId};

/// Demangled name.
pub struct SymbolName {
    pub full_name: Name,
    pub symbol_id: SymbolId,
}

/// A struct that can resolve symbol IDs into human-readable names.
pub trait Unresolver {
    fn unresolve_id(&self, symbol_id: SymbolId) -> Option<SymbolName>;
}

/// Implemented by types (like IR nodes) that expand internal IDs back into
/// human-readable names for target representation `T`.
pub trait Unresolve<T> {
    fn unresolve_symbols<U: Unresolver>(&self, unresolver: &U) -> T;
}
