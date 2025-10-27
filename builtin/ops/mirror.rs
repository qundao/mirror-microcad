// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin mirror operation.

use microcad_builtin_proc_macros::BuiltinOperation;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Mirror along a line (2D) or a plane (3D).
#[derive(BuiltinOperation)]
pub struct Mirror {
    x: Scalar,
    y: Scalar,
    z: Scalar,
    nx: Scalar,
    ny: Scalar,
    nz: Scalar,
}

impl Operation for Mirror {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            Ok(Geometry2D::Collection(geometries.mirror_2d(
                &microcad_core::geo2d::Line(
                    Point::new(self.x, self.y),
                    Point::new(self.x - self.ny, self.y + self.nx),
                ),
            )))
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries3D = model_.children.render_with_context(context)?;
            Ok(Geometry3D::Collection(geometries.mirror_3d(
                &microcad_core::geo3d::Plane {
                    p: Vec3::new(self.x, self.y, self.z),
                    n: Vec3::new(self.nx, self.ny, self.nz),
                },
            )))
        })
    }
}
