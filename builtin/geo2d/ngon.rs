// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in Ngon primitive.

use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Ngeo primitive with radius.
#[derive(Debug, Clone)]
pub struct Ngon {
    /// Number of sides.
    n: Integer,
    /// Radius.
    radius: Scalar,
}

impl Render<Geometry2D> for Ngon {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        Geometry2D::Polygon(Polygon::new(
            LineString::new(
                NgonIterator::new(self.n as u32)
                    .map(|p| p * self.radius)
                    .collect(),
            ),
            vec![],
        ))
    }
}

impl RenderWithContext<Geometry2DOutput> for Ngon {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}

impl BuiltinWorkbenchDefinition for Ngon {
    fn id() -> &'static str {
        "Ngon"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Ngon {
                n: args.get("n"),
                radius: args.get("radius"),
            })))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(n: Integer), parameter!(radius: Scalar)]
            .into_iter()
            .collect()
    }
}
