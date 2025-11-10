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
        Geometries2D::from_iter(self.iter().scan(0.0_f64, |pos, geo2d| {
            let bounds = geo2d.calc_bounds_2d();
            let dist = *bounds.distance_center_to_boundary(direction);
            let d = (*pos + dist) * direction - bounds.center();
            *pos += 2.0 * dist + *spacing;

            Some(std::rc::Rc::new(
                geo2d.transformed_2d(&Mat3::from_translation(d)),
            ))
        }))
    }
}
