// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin align operation.

use crate::*;

/// Trait to align a 2D geometry collection with spacing along an axis.
pub trait Align3D<T = Self> {
    /// Align geometry.
    fn align_3d(&self, direction: Vec3, spacing: Length) -> T;
}

impl Align3D for Geometries3D {
    fn align_3d(&self, direction: Vec3, spacing: Length) -> Self {
        let mut geometries = Vec::new();

        let mut pos = 0.0_f64;
        for geo3d in self.iter() {
            let bounds = geo3d.calc_bounds_3d();
            let center = bounds.center();
            let dist = bounds.distance_center_to_boundary(direction);
            let d = (pos + *dist) * direction - center;
            pos += 2.0 * *dist + *spacing;
            geometries.push(geo3d.transformed_3d(&Mat4::from_translation(d)));
        }

        Geometries3D::new(geometries)
    }
}
