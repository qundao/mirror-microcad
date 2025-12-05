// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinOperation3D;
use microcad_core::*;
use microcad_lang::{builtin::*, model::*, render::*};

#[derive(BuiltinOperation3D)]
pub struct Spiralize {
    height: Length,
    inner_radius: Length,
    outer_radius: Length,
    angle: Angle,
}

impl Operation for Spiralize {
    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            let mut bounds = geometries.calc_bounds_2d();
            bounds.extend_by_point(Vec2::new(0.0, 0.0)); // Add origin point.
            let radius = bounds.max_extent();
            use microcad_core::Extrude;

            let WithBounds3D { inner, bounds } = geometries.spiralize(
                *self.height,
                *self.inner_radius,
                *self.outer_radius,
                self.angle.0 / consts::PI * 0.5,
                context.current_resolution().circular_segments(radius) as usize,
            );

            Ok(WithBounds3D::new(inner.into(), bounds))
        })
    }
}
