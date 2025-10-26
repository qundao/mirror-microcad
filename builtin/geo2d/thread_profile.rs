// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Metric thread profile.

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// ISO metric thread profile (one period).
#[derive(BuiltinPrimitive2D)]
pub struct ThreadProfile {
    /// Pitch (distance between two threads).
    pub p: Scalar,
}

impl Render<Geometry2D> for ThreadProfile {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        use geo::Coord;
        // Theoretical height of the 60° thread
        let h = (3f64.sqrt() / 2.0) * self.p;
        // Effective height (after truncations at top and bottom)
        let h1 = 5.0 * h / 8.0;
        // Flat widths
        let flat_top = self.p / 8.0;
        let flat_bottom = self.p / 4.0;

        // Build the polyline (one period of the external thread)
        let mut pts = Vec::with_capacity(4);

        // Start at bottom left (root)
        pts.push(Coord { x: 0.0, y: 0.0 });

        // Left flank up to the crest
        let dx_flank = self.p / 2.0 - flat_bottom / 2.0;
        let dy_flank = h1;
        pts.push(Coord {
            x: dx_flank,
            y: dy_flank,
        });

        // Flat crest
        let x_top_left = dx_flank;
        let x_top_right = x_top_left + flat_top;
        pts.push(Coord {
            x: x_top_right,
            y: dy_flank,
        });

        // Right flank down
        let dx_flank2 = dx_flank;
        pts.push(Coord {
            x: x_top_right + dx_flank2,
            y: 0.0,
        });

        Polygon::new(LineString::new(pts), vec![]).into()
    }
}

impl RenderWithContext<Geometry2DOutput> for ThreadProfile {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
