// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render hash functionality.

use derive_more::Deref;

/// Render hash type.
pub type HashId = u64;

pub use rustc_hash::FxHashMap as HashMap;
pub use rustc_hash::FxHashSet as HashSet;
pub use rustc_hash::FxHasher as Hasher;
use serde::Serialize;

/// Trait to implement for typed that contain a pre-computed hash value.
pub trait ComputedHash {
    /// Return computed hash value.
    fn computed_hash(&self) -> HashId;
}

/// Generic wrapper that contains the hashed value.
#[derive(Deref, Debug, Clone, Serialize)]
#[serde(bound(serialize = "T: Serialize"))]
pub struct Hashed<T: std::hash::Hash> {
    #[deref]
    inner: T,
    hash: HashId,
}

impl<T: std::hash::Hash> Hashed<T> {
    /// Create a new wrapper with hashed.
    pub fn new(inner: T) -> Self {
        let mut hasher = Hasher::default();
        inner.hash(&mut hasher);
        Self {
            inner,
            hash: std::hash::Hasher::finish(&hasher),
        }
    }

    /// Transforms the inner value and recalculates the hash for the new value.
    pub fn map<U: std::hash::Hash, F>(self, f: F) -> Hashed<U>
    where
        F: FnOnce(T) -> U,
    {
        // Transform the value
        let new_inner = f(self.inner);

        // Re-hash the new value to ensure the HashId stays in sync
        Hashed::new(new_inner)
    }

    /// Return inner value.
    pub fn value(&self) -> &T {
        &self.inner
    }
}

impl<T: std::hash::Hash> ComputedHash for Hashed<T> {
    fn computed_hash(&self) -> HashId {
        self.hash
    }
}
