// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin offset operation.

use microcad_builtin_proc_macros::BuiltinOperation2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Transform a geometry whose boundary is offset the specified distance from the input.
///
/// Only implemented for 2D geometries at the moment.
#[derive(BuiltinOperation2D)]
pub struct Buffer {
    /// Specified distance in mm.
    distance: Length,
}

impl Operation for Buffer {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            use geo::Buffer;
            Ok(Geometry2D::MultiPolygon(geometries.buffer(*self.distance)))
        })
    }
}
