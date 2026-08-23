// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! lang-base/src/builtin.rs

use derive_more::{Debug, Display};
use microcad_hash::HashId;
use serde::{Deserialize, Serialize};

use crate::VersionAnnotation;

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
    /// Full name of the built-in, e.g. `__mu::geo2d::Circle`.
    pub name: &'static str,
    /// The hash ID of the built-in, generated from it's full-name.
    pub id: BuiltinId,
    /// Optional documentation.
    pub doc: Option<&'static str>,
    /// Function to get version annotation
    pub version_annotation: Option<fn() -> VersionAnnotation>,
}

impl BuiltinInfo {
    pub const fn new(name: &'static str) -> Self {
        Self {
            id: BuiltinId::from_name(name),
            name,
            doc: None,
            version_annotation: None,
        }
    }

    pub const fn with_doc(mut self, doc: &'static str) -> Self {
        if !doc.is_empty() {
            self.doc = Some(doc);
        }
        self
    }

    pub const fn with_version_annotation(
        mut self,
        version_annotation: fn() -> VersionAnnotation,
    ) -> Self {
        self.version_annotation = Some(version_annotation);
        self
    }
}

/// Item name methods.
impl BuiltinInfo {
    /// Return the Id of the built-in
    pub const fn id(&self) -> BuiltinId {
        self.id
    }

    /// Returns the item name (e.g., `"Circle"` from `"__mu::geo2d::Circle"`).
    /// Returns `None` if the name only contains a root/module without a distinct item.
    pub fn item_name(&self) -> Option<&'static str> {
        let parts: Vec<&'static str> = self.name.split("::").collect();
        if parts.len() >= 1 {
            parts.last().copied()
        } else {
            None
        }
    }
}
