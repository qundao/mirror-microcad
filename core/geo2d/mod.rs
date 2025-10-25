// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

mod bounds;
mod circle;
mod collection;
mod geometry;
mod line;
mod mirror;
mod primitives;
mod size;

use crate::*;

pub use bounds::*;
pub use circle::*;
pub use collection::*;
use geo::AffineTransform;
pub use geometry::*;
pub use line::*;
pub use mirror::*;
pub use primitives::*;
pub use size::*;

/// Trait to return all points of 2D geometry.
pub trait FetchPoints2D {
    /// Returns all points.
    fn fetch_points_2d(&self) -> Vec<Vec2>;
}

/// Transformed version of a 2D geometry.
pub trait Transformed2D<T = Self> {
    /// Transform from matrix.
    fn transformed_2d(&self, mat: &Mat3) -> T;
}

/// Convert a [`Mat3`]` into an affine transform.
pub(crate) fn mat3_to_affine_transform(mat: &Mat3) -> AffineTransform {
    geo::AffineTransform::new(mat.x.x, mat.y.x, mat.z.x, mat.x.y, mat.y.y, mat.z.y)
}

/// Iterator over `n` points of a regular convex polygon (Ngon).
pub struct NgonIterator {
    /// Number of points.
    n: u32,
    /// Current point.
    i: u32,
}

impl NgonIterator {
    /// Create new iterator.
    pub fn new(n: u32) -> Self {
        Self { n, i: 0 }
    }
}

impl Iterator for NgonIterator {
    type Item = geo::Coord<Scalar>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.n {
            use std::f64::consts::PI;
            let angle = 2.0 * PI * (self.i as f64) / (self.n as f64);
            self.i += 1;
            Some(geo::coord! {x: angle.cos(), y: angle.sin() })
        } else {
            None
        }
    }
}
