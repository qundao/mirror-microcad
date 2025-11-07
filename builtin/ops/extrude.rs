// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinOperation3D;
use microcad_core::*;
use microcad_lang::{builtin::*, model::*, render::*};

/// Linear extrude operation.
#[derive(BuiltinOperation3D)]
pub struct Extrude {
    /// Extrusion height in mm (in Z direction).
    height: Scalar,
    /// Scale in X direction.
    scale_x: Scalar,
    /// Scale in Y direction.
    scale_y: Scalar,
    /// Twist angle in °0 direction.
    twist: Angle,
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
            let mesh = geometries.extrude(Extrusion::Linear {
                height: self.height,
                scale_x: self.scale_x,
                scale_y: self.scale_y,
                twist: self.twist,
            });
            Ok(WithBounds3D::new(mesh.inner.into(), mesh.bounds))
        })
    }
}
