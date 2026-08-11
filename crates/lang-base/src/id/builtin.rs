// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! lang-base/src/builtin.rs

use derive_more::Display;
use microcad_hash::HashId;
use serde::{Deserialize, Serialize};

/// Strongly-typed wrapper around raw builtin u64 hashes
#[derive(
    Debug, Copy, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct BuiltinId(pub HashId);

impl BuiltinId {
    /// FNV-1a compile-time hashing function
    pub const fn from_name(name: &str) -> Self {
        Self(HashId::compile_time_hash(name))
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
