// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin hull operation.

use microcad_builtin_proc_macros::BuiltinOperation;
use microcad_lang::{builtin::*, render::*};

#[derive(BuiltinOperation)]
pub struct Hull;

impl Operation for Hull {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model_ = model.borrow();
            let geometry: Geometry2DOutput = model_.children.render_with_context(context)?;

            Ok(geometry.inner.hull())
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometry: Geometry3DOutput = model_.children.render_with_context(context)?;

            Ok(geometry.inner.hull())
        })
    }
}
