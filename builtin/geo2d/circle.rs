// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// A circle at origin.
#[derive(BuiltinPrimitive2D)]
pub struct Circle {
    /// Circle radius.
    radius: Scalar,
}

impl Render<Geometry2D> for Circle {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        Geometry2D::Polygon(microcad_core::Circle::circle_polygon(
            self.radius,
            resolution,
        ))
    }
}

impl RenderWithContext<Geometry2DOutput> for Circle {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
