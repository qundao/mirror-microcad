// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in Ngon primitive.

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Ngon primitive with radius.
#[derive(BuiltinPrimitive2D)]
pub struct Ngon {
    /// Number of sides.
    n: Integer,
    /// Radius.
    radius: Length,
}

impl Render<Geometry2D> for Ngon {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        Geometry2D::Polygon(Polygon::new(
            LineString::new(
                NgonIterator::new(self.n as u32)
                    .map(|p| p * *self.radius)
                    .collect(),
            ),
            vec![],
        ))
    }
}

impl RenderWithContext<Geometry2DOutput> for Ngon {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
