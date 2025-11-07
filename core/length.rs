// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Length type.

use super::Scalar;
use derive_more::{Deref, DerefMut};

/// A length in millimeters.
#[derive(Clone, Deref, DerefMut)]
pub struct Length(pub Scalar);

impl Length {
    /// Return a new length from millimeters.
    pub fn mm(mm: Scalar) -> Self {
        Self(mm)
    }
}
