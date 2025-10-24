// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::Primitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Pie geometry with offset.
#[derive(Primitive2D, Debug, Clone)]
pub struct Pie {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Start angle.
    pub start_angle: Angle,

    /// End angle.
    pub end_angle: Angle,
}

impl Pie {
    /// A pie is a circle when `offset_angle >= 360°`.
    pub fn is_circle(&self) -> bool {
        self.offset_angle() >= cgmath::Deg(360.0).into()
    }

    /// Calculate offset angle.
    pub fn offset_angle(&self) -> Angle {
        self.end_angle - self.start_angle
    }
}

impl Render<Geometry2D> for Pie {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        use std::f64::consts::PI;
        let offset_angle = self.offset_angle();
        let n =
            (resolution.circular_segments(self.radius) as f64 * (offset_angle.0 / PI / 2.0)) as u32;

        let points = if !self.is_circle() && n > 0 {
            (0..=n)
                .map(|i| {
                    let angle = self.start_angle + offset_angle * (i as f64) / (n as f64);
                    geo::coord!(x: angle.0.cos(), y: angle.0.sin()) * self.radius
                })
                .chain(
                    // Add center point.
                    [geo::coord!(x: 0.0 as Scalar, y: 0.0 as Scalar)]
                        .iter()
                        .cloned(),
                )
                .collect()
        } else {
            (0..n)
                .map(|i| {
                    let angle = 2.0 * PI * (i as f64) / (n as f64);
                    geo::coord!(x: angle.cos(), y: angle.sin()) * self.radius
                })
                .collect()
        };

        Geometry2D::Polygon(Polygon::new(LineString::new(points), vec![]))
    }
}

impl RenderWithContext<Geometry2DOutput> for Pie {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
