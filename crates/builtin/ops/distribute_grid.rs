// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin align operation.

use microcad_builtin_proc_macros::BuiltinOperation;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Distribute a 2D or 3D geometry on a grid on the Z plane.
#[derive(BuiltinOperation)]
pub struct DistributeGrid {
    /// X offset of the grid.
    x: Length,
    /// X offset of the grid.
    y: Length,
    /// Overall width of the grid.
    width: Length,
    /// Overall height of the grid.
    height: Length,
    /// Number of rows in the grid.
    rows: Integer,
    /// Number of columns in the grid.
    columns: Integer,
}

impl DistributeGrid {
    fn rect(&self) -> Rect {
        let (x, y, w, h) = (*self.x, *self.y, *self.width, *self.height);
        Rect::new(
            geo::coord! { x: x, y: y },
            geo::coord! { x: x + w, y: y + h },
        )
    }
}

impl Operation for DistributeGrid {
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

            use microcad_core::traits::DistributeGrid;
            Ok(Geometry2D::Collection(
                Geometries2D::new(geometries).distribute_grid(self.rect(), self.rows, self.columns),
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

            use microcad_core::traits::DistributeGrid;
            Ok(Geometry3D::Collection(
                Geometries3D::new(geometries).distribute_grid(self.rect(), self.rows, self.columns),
            ))
        })
    }
}
