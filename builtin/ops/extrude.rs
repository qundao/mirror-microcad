// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinOperation3D;
use microcad_core::*;
use microcad_lang::{builtin::*, model::*, render::*};

#[derive(BuiltinOperation3D)]
pub struct Extrude {
    height: Scalar,
    n_divisions: Integer,
    twist_degrees: Scalar,
    scale_top_x: Scalar,
    scale_top_y: Scalar,
}

impl Operation for Extrude {
    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;

            use microcad_core::Extrude;
            let mesh = geometries.linear_extrude(self.height);
            Ok(WithBounds3D::new(mesh.inner.into(), mesh.bounds))
        })
    }
}
