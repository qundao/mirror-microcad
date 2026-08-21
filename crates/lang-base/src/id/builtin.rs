// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! lang-base/src/builtin.rs

use derive_more::{Debug, Display};
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

/// Information about a built-in symbol.
///
/// Each built-in symbol has a [`BuiltinInfo`] as property.
#[derive(Debug, Clone, Display)]
#[debug("{name}")]
#[display("{name}@{id}")]
pub struct BuiltinInfo {
    pub name: &'static str,
    pub id: BuiltinId,
    pub doc: Option<&'static str>,
}

impl BuiltinInfo {
    pub const fn new(name: &'static str) -> Self {
        Self {
            id: BuiltinId::from_name(name),
            name,
            doc: None,
        }
    }

    pub const fn with_doc(mut self, doc: &'static str) -> Self {
        if !doc.is_empty() {
            self.doc = Some(doc);
        }
        self
    }

    pub const fn hash(&self) -> HashId {
        self.id().0
    }

    pub const fn id(&self) -> BuiltinId {
        self.id
    }
}
