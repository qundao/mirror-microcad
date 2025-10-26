// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mirror 2D geometries.

use super::*;
use cgmath::InnerSpace;

/// Reflects a 2D geometry along a line.
pub trait Reflect2D<T = Self> {
    /// Mirror a 2D geometry.
    fn reflect_2d(&self, l: &Line) -> T;
}

/// Mirrors a 2D geometry, keeping the original (in contrast to `reflect`).
pub trait Mirror2D<T = Self>: Reflect2D<T> + Clone + Into<Geometry2D> {
    /// Mirror operation.
    fn mirror_2d(&self, l: &Line) -> Geometries2D {
        let orig: Geometry2D = self.clone().into();
        let refl: Geometry2D = orig.reflect_2d(l);
        Geometries2D::new(vec![orig, refl])
    }
}

impl Reflect2D for Point {
    fn reflect_2d(&self, l: &Line) -> Self {
        let d = l.vec();
        let t = ((self.x() - l.0.x()) * d.x + (self.y() - l.0.y()) * d.y) / d.magnitude();
        let q = 2.0 * (Vec2::from(l.0.0.x_y()) + t * d);
        Self::new(q.x - self.x(), q.y - self.y())
    }
}

impl Reflect2D for geo::Coord {
    fn reflect_2d(&self, l: &Line) -> Self {
        Point::from(self.x_y()).reflect_2d(l).into()
    }
}

impl Reflect2D for LineString {
    fn reflect_2d(&self, l: &Line) -> Self {
        Self::new(self.0.iter().map(|c| c.reflect_2d(l)).rev().collect())
    }
}

impl Mirror2D for LineString {}

impl Reflect2D for MultiLineString {
    fn reflect_2d(&self, l: &Line) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|line_string| line_string.reflect_2d(l))
                .collect(),
        )
    }
}

impl Mirror2D for MultiLineString {}

impl Reflect2D for Polygon {
    fn reflect_2d(&self, l: &Line) -> Self {
        Self::new(
            self.exterior().reflect_2d(l),
            self.interiors()
                .iter()
                .map(|interior| interior.reflect_2d(l))
                .collect(),
        )
    }
}

impl Mirror2D for Polygon {}

impl Reflect2D for MultiPolygon {
    fn reflect_2d(&self, l: &Line) -> Self {
        Self::new(self.0.iter().map(|polygon| polygon.reflect_2d(l)).collect())
    }
}

impl Mirror2D for MultiPolygon {}

impl Reflect2D<Polygon> for Rect {
    fn reflect_2d(&self, l: &Line) -> Polygon {
        self.to_polygon().reflect_2d(l)
    }
}

impl Mirror2D<Polygon> for Rect {}

impl Reflect2D for Line {
    fn reflect_2d(&self, l: &Line) -> Self {
        Self(self.1.reflect_2d(l), self.0.reflect_2d(l))
    }
}

impl Mirror2D for Line {}

impl Reflect2D for Geometry2D {
    fn reflect_2d(&self, l: &Line) -> Self {
        match &self {
            Geometry2D::LineString(line_string) => {
                Geometry2D::LineString(line_string.reflect_2d(l))
            }
            Geometry2D::MultiLineString(multi_line_string) => {
                Geometry2D::MultiLineString(multi_line_string.reflect_2d(l))
            }
            Geometry2D::Polygon(polygon) => Geometry2D::Polygon(polygon.reflect_2d(l)),
            Geometry2D::MultiPolygon(multi_polygon) => {
                Geometry2D::MultiPolygon(multi_polygon.reflect_2d(l))
            }
            Geometry2D::Rect(rect) => Geometry2D::Polygon(rect.reflect_2d(l)),
            Geometry2D::Line(line) => Geometry2D::Line(line.reflect_2d(l)),
            Geometry2D::Collection(collection) => Geometry2D::Collection(collection.reflect_2d(l)),
        }
    }
}

impl Mirror2D for Geometry2D {}

impl Reflect2D for Geometries2D {
    fn reflect_2d(&self, l: &Line) -> Self {
        Self::from_iter(
            self.iter()
                .map(|geometry| std::rc::Rc::new(geometry.as_ref().reflect_2d(l))),
        )
    }
}

impl Mirror2D for Geometries2D {}
