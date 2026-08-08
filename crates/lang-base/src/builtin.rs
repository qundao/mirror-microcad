// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! lang-base/src/builtin.rs

use serde::{Deserialize, Serialize};

/// Strongly-typed wrapper around raw builtin u64 hashes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BuiltinId(pub u64);

impl BuiltinId {
    /// FNV-1a compile-time hashing function
    pub const fn from_name(name: &str) -> Self {
        Self(microcad_hash::fnv1a_hash(name))
    }
}

impl<'a> From<&'a str> for BuiltinId {
    fn from(name: &'a str) -> Self {
        Self::from_name(name)
    }
}

// Allow conversion directly from String
impl From<String> for BuiltinId {
    #[inline]
    fn from(name: String) -> Self {
        Self::from_name(name.as_str())
    }
}

// Print cleanly as hex in debug/format strings (e.g. BuiltinId(0x4A8F...))
impl std::fmt::Display for BuiltinId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016X}", self.0)
    }
}

/// Builtin name
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltinName {
    pub name: &'static str,
    pub id: BuiltinId,
}

impl BuiltinName {
    /// Create a new BuiltinName (const)
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            id: BuiltinId::from_name(name),
        }
    }
}

impl From<&'static str> for BuiltinName {
    fn from(name: &'static str) -> Self {
        BuiltinName {
            name,
            id: BuiltinId::from_name(name),
        }
    }
}

impl std::fmt::Display for BuiltinName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.id)
    }
}
