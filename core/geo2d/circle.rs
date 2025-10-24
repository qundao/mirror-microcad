// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

use crate::*;

/// Circle with offset.
#[derive(Debug, Clone)]
pub struct Circle {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Offset.
    pub offset: Vec2,
}

impl CalcBounds2D for Circle {
    fn calc_bounds_2d(&self) -> Bounds2D {
        use geo::Coord;

        if self.radius > 0.0 {
            let r = Vec2::new(self.radius, self.radius);
            let min: (Scalar, Scalar) = (self.offset - r).into();
            let max: (Scalar, Scalar) = (self.offset + r).into();

            Some(Rect::new(Coord::from(min), Coord::from(max)))
        } else {
            None
        }
        .into()
    }
}

impl FetchPoints2D for Circle {
    fn fetch_points_2d(&self) -> Vec<Vec2> {
        vec![self.offset]
    }
}

impl Render<Polygon> for Circle {
    fn render(&self, resolution: &RenderResolution) -> Polygon {
        let n = resolution.circular_segments(self.radius);
        let points = NgonIterator::new(n)
            .map(|p| {
                geo::coord!(x: self.radius * p.x + self.offset.x, y: self.radius * p.y + self.offset.y)
            })
            .collect();

        Polygon::new(LineString::new(points), vec![])
    }
}
