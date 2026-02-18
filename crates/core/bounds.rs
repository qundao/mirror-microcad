// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generic bounds.

/// Bounds.
#[derive(Debug, Clone)]
pub struct Bounds<T> {
    /// Minimum corner.
    pub min: T,
    /// Maximum corner.
    pub max: T,
}

impl<T> Bounds<T> {
    /// Create new bounds (unvalidated).
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Minimum and maximum corner.
    pub fn min_max(&self) -> (&T, &T) {
        (&self.min, &self.max)
    }
}
