// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin align operation.

use microcad_builtin_proc_macros::BuiltinOperation;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

#[derive(BuiltinOperation)]
pub struct Align;

impl Operation for Align {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            use microcad_core::traits::Align;
            Ok(Geometry2D::Collection(geometries).align())
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries3D = model_.children.render_with_context(context)?;
            use microcad_core::traits::Align;
            Ok(Geometry3D::Collection(geometries).align())
        })
    }
}
