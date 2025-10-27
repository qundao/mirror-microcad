// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Pie geometry with offset.
#[derive(BuiltinPrimitive2D)]
pub struct Pie {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Start angle.
    pub start: Angle,

    /// End angle.
    pub end: Angle,
}

impl Pie {
    /// A pie is a circle when `offset_angle >= 360°`.
    pub fn is_circle(&self) -> bool {
        self.offset_angle() >= cgmath::Deg(360.0).into()
    }

    /// Calculate offset angle.
    pub fn offset_angle(&self) -> Angle {
        self.end - self.start
    }
}

impl Render<Geometry2D> for Pie {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        use std::f64::consts::PI;
        let offset = self.offset_angle();
        let n = (resolution.circular_segments(self.radius) as f64 * (offset.0 / PI / 2.0)) as u32;

        let points = if !self.is_circle() && n > 0 {
            (0..=n)
                .map(|i| {
                    let angle = self.start + offset * (i as f64) / (n as f64);
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

        Polygon::new(LineString::new(points), vec![]).into()
    }
}

impl RenderWithContext<Geometry2DOutput> for Pie {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
