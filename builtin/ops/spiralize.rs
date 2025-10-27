// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, model::*, render::*};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Spiralize {
    height: Scalar,
    inner_radius: Scalar,
    outer_radius: Scalar,
    turns: Scalar,
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
                self.height,
                self.inner_radius,
                self.outer_radius,
                self.turns,
                context.current_resolution().circular_segments(radius) as usize,
            );

            Ok(WithBounds3D::new(inner.into(), bounds))
        })
    }
}

impl BuiltinWorkbenchDefinition for Spiralize {
    fn id() -> &'static str {
        "spiralize"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn output_type() -> OutputType {
        OutputType::Geometry3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(Spiralize {
                height: args.get("height"),
                inner_radius: args.get("inner_radius"),
                outer_radius: args.get("outer_radius"),
                turns: args.get("turns"),
            })))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(height: Scalar),
            parameter!(inner_radius: Scalar),
            parameter!(outer_radius: Scalar),
            parameter!(turns: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
