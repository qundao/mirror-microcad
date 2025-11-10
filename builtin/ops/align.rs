// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin align operation.

use microcad_builtin_proc_macros::BuiltinOperation;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

#[derive(BuiltinOperation)]
pub struct Align {
    /// x Direction.
    x: Scalar,
    /// y Direction.
    y: Scalar,
    /// z Direction.
    z: Scalar,
    /// Spacing.
    spacing: Length,
}

impl Operation for Align {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model = model.into_group().unwrap_or(model);
            let model_ = model.borrow();

            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            use microcad_core::geo2d::Align2D;
            Ok(Geometry2D::Collection(
                geometries.align_2d(Vec2::new(self.x, self.y), self.spacing),
            ))
        })
    }

    fn process_3d(&self, _context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        todo!()
    }
}
