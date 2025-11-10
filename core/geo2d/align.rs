// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin align operation.

use crate::*;

/// Trait to align a 2D geometry collection with spacing along an axis.
pub trait Align2D<T = Self> {
    /// Align geometry.
    fn align_2d(&self, direction: Vec2, spacing: Length) -> T;
}

impl Align2D for Geometries2D {
    fn align_2d(&self, direction: Vec2, spacing: Length) -> Self {
        let mut geometries = Vec::new();

        let mut pos = 0.0_f64;
        for geo2d in self.iter() {
            let bounds = geo2d.calc_bounds_2d();
            let center = bounds.center();
            let dist = bounds.distance_center_to_boundary(direction);
            let d = (pos + *dist) * direction - center;
            pos += 2.0 * *dist + *spacing;
            geometries.push(geo2d.transformed_2d(&Mat3::from_translation(d)));
        }

        Geometries2D::new(geometries)
    }
}
