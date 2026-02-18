// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// A rectangle.
#[derive(BuiltinPrimitive2D)]
pub struct Rect {
    /// X position (left side) of the rectangle.
    x: Length,
    /// Y position (bottom side) of the rectangle.
    y: Length,
    /// Width of the rectangle.
    width: Length,
    /// Height of the rectangle.
    height: Length,
}

impl Render<Geometry2D> for Rect {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        let (x, y, w, h) = (*self.x, *self.y, *self.width, *self.height);
        Geometry2D::Rect(geo2d::Rect::new(
            geo::coord! {x: x, y: y},
            geo::coord! {x: x + w, y: y + h},
        ))
    }
}

impl RenderWithContext<Geometry2DOutput> for Rect {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
