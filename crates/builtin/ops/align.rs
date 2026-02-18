// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
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
            let mut geometries = Vec::new();
            model
                .multiplicity_descendants()
                .try_for_each(|model| -> RenderResult<()> {
                    let model_geometries: Geometries2D =
                        model.borrow().children.render_with_context(context)?;
                    geometries.extend(model_geometries.iter().map(|geo| geo.as_ref().clone()));
                    Ok(())
                })?;
            use microcad_core::geo2d::Align2D;
            Ok(Geometry2D::Collection(
                Geometries2D::new(geometries).align_2d(Vec2::new(self.x, self.y), self.spacing),
            ))
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model = model.into_group().unwrap_or(model);
            let mut geometries = Vec::new();
            model
                .multiplicity_descendants()
                .try_for_each(|model| -> RenderResult<()> {
                    let model_geometries: Geometries3D =
                        model.borrow().children.render_with_context(context)?;
                    geometries.extend(model_geometries.iter().map(|geo| geo.as_ref().clone()));
                    Ok(())
                })?;
            use microcad_core::geo3d::Align3D;
            Ok(Geometry3D::Collection(
                Geometries3D::new(geometries)
                    .align_3d(Vec3::new(self.x, self.y, self.z), self.spacing),
            ))
        })
    }
}
