// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::Primitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

#[derive(Primitive2D)]
pub struct Rect {
    x: Scalar,
    y: Scalar,
    width: Scalar,
    height: Scalar,
}

impl Render<Geometry2D> for Rect {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        use geo::coord;
        Geometry2D::Rect(geo2d::Rect::new(
            coord! {x: self.x, y: self.y},
            coord! {x: self.x + self.width, y: self.y + self.height},
        ))
    }
}

impl RenderWithContext<Geometry2DOutput> for Rect {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
