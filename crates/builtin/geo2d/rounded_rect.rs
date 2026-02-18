// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// A rounded rectangle with a radius for each corner.
#[derive(BuiltinPrimitive2D)]
pub struct RoundedRect {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    radius_bottom_left: Length,
    radius_bottom_right: Length,
    radius_top_left: Length,
    radius_top_right: Length,
}

impl Render<Geometry2D> for RoundedRect {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        use geo::Coord;

        // Compute the corners of the rectangle
        let x0 = *self.x;
        let y0 = *self.y;
        let x1 = *self.x + *self.width;
        let y1 = *self.y + *self.height;

        // clamp each radius so that no radius exceeds half the width or half the height
        let max_r = (self.width.min(*self.height)) * 0.5;
        let r_bl = self.radius_bottom_left.min(max_r);
        let r_br = self.radius_bottom_right.min(max_r);
        let r_tl = self.radius_top_left.min(max_r);
        let r_tr = self.radius_top_right.min(max_r);

        // Determine how many segments for each quarter-circle corner based on resolution
        // (You may choose a better heuristic for segment count)

        // Helper to generate arc points from angle_start to angle_end with centre (cx, cy), radius r
        fn arc_points(
            c: (Scalar, Scalar),
            r: Scalar,
            angle_start: Scalar,
            angle_end: Scalar,
            resolution: &RenderResolution,
        ) -> Vec<Coord<Scalar>> {
            let segments = (resolution.circular_segments(r) / 4) as usize;
            (0..=segments)
                .map(|i| {
                    let t = i as Scalar / segments as Scalar;
                    let theta = angle_start + (angle_end - angle_start) * t;
                    (c.0 + r * theta.cos(), c.1 + r * theta.sin()).into()
                })
                .collect()
        }

        let mut pts = Vec::new();

        // Bottom-edge from bottom-left corner to bottom-right corner (between corner arcs)
        // Start at bottom-left arc end
        pts.extend(arc_points(
            (x0 + r_bl, y0 + r_bl),
            r_bl,
            consts::PI,
            consts::PI * 3.0 / 2.0,
            resolution,
        ));
        // Bottom edge straight from after bottom-left arc to start of bottom-right arc
        pts.push((x1 - r_br, y0).into());
        // Bottom-right corner arc
        pts.extend(arc_points(
            (x1 - r_br, y0 + r_br),
            r_br,
            consts::PI * 3.0 / 2.0,
            consts::PI * 2.0,
            resolution,
        ));
        // Right edge
        pts.push((x1, y1 - r_tr).into());

        // Top-right arc
        pts.extend(arc_points(
            (x1 - r_tr, y1 - r_tr),
            r_tr,
            0.0,
            consts::PI / 2.0,
            resolution,
        ));
        // Top edge
        pts.push((x0 + r_tl, y1).into());

        // Top-left arc
        pts.extend(arc_points(
            (x0 + r_tl, y1 - r_tl),
            r_tl,
            consts::PI / 2.0,
            consts::PI,
            resolution,
        ));
        // Left edge (closing)
        pts.push((x0, y0 + r_bl).into());

        // Close the loop by ensuring the first point equals the last
        if let Some(first) = pts.first().cloned() {
            pts.push(first);
        }

        Geometry2D::Polygon(Polygon::new(LineString::from(pts), vec![]))
    }
}

impl RenderWithContext<Geometry2DOutput> for RoundedRect {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
