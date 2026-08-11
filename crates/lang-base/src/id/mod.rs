// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module to handle and demangle IDs use in µcad language.

mod builtin;
mod unresolve;

use derive_more::From;
pub use microcad_hash::{HashId, hash_id};

pub use unresolve::{Unresolve, Unresolver};

pub use builtin::{BuiltinId, BuiltinName};

use serde::{Deserialize, Serialize};

/// Name type (base of all identifiers)
pub type Name = crate::CompactString;

/// Symbol id
#[derive(Debug, Hash, PartialEq, From, Clone, Serialize, Deserialize)]
pub enum SymbolId {
    /// A builtin symbol.
    Builtin(BuiltinId),
    /// A definition within the current package/module (e.g. a workbench or function).
    Item(HashId),
    /// A symbol in an external package (e.g. `std`)
    External { package_name: Name, id: HashId },
}
