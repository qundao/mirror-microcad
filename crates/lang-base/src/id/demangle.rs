// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Name, SymbolId};

/// Demangled name.
pub struct DemangledName {
    pub full_name: Name,
    pub symbol_id: SymbolId,
}

/// A struct that can resolve symbol IDs into human-readable names.
pub trait Demangler {
    fn demangle_id(&self, symbol_id: SymbolId) -> Option<DemangledName>;
}

/// Implemented by types that can demangle themselves, like IR nodes.
/// into a target representation `T` using a `Demangler`.
pub trait Demangle<T> {
    fn demangle<D: Demangler>(&self, demangler: &D) -> T;
}
