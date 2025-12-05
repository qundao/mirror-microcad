// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Built-in line primitive.
#[derive(BuiltinPrimitive2D)]
pub struct Line {
    x0: Scalar,
    y0: Scalar,
    x1: Scalar,
    y1: Scalar,
}

impl Render<Geometry2D> for Line {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        Geometry2D::Line(geo2d::Line(
            (self.x0, self.y0).into(),
            (self.x1, self.y1).into(),
        ))
    }
}

impl RenderWithContext<Geometry2DOutput> for Line {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
