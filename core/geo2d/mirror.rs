// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mirror 2D geometries.

use super::*;
use cgmath::InnerSpace;

/// Mirror along a line.
pub trait Mirror2D<T = Self> {
    /// Mirror a 2D geometry.
    fn mirror_2d(&self, l: &Line) -> T;
}

impl Mirror2D for Point {
    fn mirror_2d(&self, l: &Line) -> Self {
        let d = l.vec();
        let t = ((self.x() - l.0.x()) * d.x + (self.y() - l.0.y()) * d.y) / d.magnitude();
        let q = 2.0 * (Vec2::from(l.0.0.x_y()) + t * d);
        Self::new(q.x - self.x(), q.y - self.y())
    }
}

impl Mirror2D for geo::Coord {
    fn mirror_2d(&self, l: &Line) -> Self {
        Point::from(self.x_y()).mirror_2d(l).into()
    }
}

impl Mirror2D for LineString {
    fn mirror_2d(&self, l: &Line) -> Self {
        Self::new(self.0.iter().map(|c| c.mirror_2d(l)).rev().collect())
    }
}

impl Mirror2D for MultiLineString {
    fn mirror_2d(&self, l: &Line) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|line_string| line_string.mirror_2d(l))
                .collect(),
        )
    }
}

impl Mirror2D for Polygon {
    fn mirror_2d(&self, l: &Line) -> Self {
        Self::new(
            self.exterior().mirror_2d(l),
            self.interiors()
                .iter()
                .map(|interior| interior.mirror_2d(l))
                .collect(),
        )
    }
}

impl Mirror2D for MultiPolygon {
    fn mirror_2d(&self, l: &Line) -> Self {
        Self::new(self.0.iter().map(|polygon| polygon.mirror_2d(l)).collect())
    }
}

impl Mirror2D<Polygon> for Rect {
    fn mirror_2d(&self, l: &Line) -> Polygon {
        self.to_polygon().mirror_2d(l)
    }
}

impl Mirror2D for Line {
    fn mirror_2d(&self, l: &Line) -> Self {
        Self(self.1.mirror_2d(l), self.0.mirror_2d(l))
    }
}

impl Mirror2D for Geometries2D {
    fn mirror_2d(&self, l: &Line) -> Self {
        Self::from_iter(
            self.iter()
                .map(|geometry| std::rc::Rc::new(geometry.as_ref().mirror_2d(l))),
        )
    }
}

impl Mirror2D for Geometry2D {
    fn mirror_2d(&self, l: &Line) -> Self {
        match &self {
            Geometry2D::LineString(line_string) => Geometry2D::LineString(line_string.mirror_2d(l)),
            Geometry2D::MultiLineString(multi_line_string) => {
                Geometry2D::MultiLineString(multi_line_string.mirror_2d(l))
            }
            Geometry2D::Polygon(polygon) => Geometry2D::Polygon(polygon.mirror_2d(l)),
            Geometry2D::MultiPolygon(multi_polygon) => {
                Geometry2D::MultiPolygon(multi_polygon.mirror_2d(l))
            }
            Geometry2D::Rect(rect) => Geometry2D::Polygon(rect.mirror_2d(l)),
            Geometry2D::Line(line) => Geometry2D::Line(line.mirror_2d(l)),
            Geometry2D::Collection(collection) => Geometry2D::Collection(collection.mirror_2d(l)),
        }
    }
}
