// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

use microcad_core::geo2d::bezier::{Bezier, Subpath, subpath_to_geo_polygon};

/// Text geometry with a height.
#[derive(Clone)]
pub struct BezierPolygon {
    /// Bezier paths as string.
    ///
    /// It contains all handles values in separated by whitespace.
    pub path: String,
}

impl BezierPolygon {
    pub fn parse_continuous_cubic_subpath(input: &str) -> Result<Subpath, String> {
        let numbers: Vec<f64> = input
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Parsing error: {}", e))?;

        if numbers.len() < 2 || (numbers.len() - 2) % 6 != 0 {
            return Err(
                "String length does not match an initial point + 6-tuple cubic segments."
                    .to_string(),
            );
        }

        let mut segments = Vec::new();
        let mut current_start = (numbers[0], numbers[1]);

        // Skip the first 2 elements (the initial start point) and chunk the rest by 6
        for chunk in numbers[2..].chunks_exact(6) {
            let control1 = (chunk[0], chunk[1]);
            let control2 = (chunk[2], chunk[3]);
            let end_point = (chunk[4], chunk[5]);

            segments.push(Bezier::from_cubic_dvec2(
                current_start.into(),
                control1.into(),
                control2.into(),
                end_point.into(),
            ));

            current_start = end_point; // Next segment starts where this one ended
        }

        Ok(Subpath::from_beziers(&segments, false))
    }
}

impl Render<Geometry2D> for BezierPolygon {
    fn render(&self, _resolution: &RenderResolution) -> Geometry2D {
        let subpath = Self::parse_continuous_cubic_subpath(&self.path).expect("A valid subpath");

        subpath_to_geo_polygon(&subpath, 20).into()
    }
}

impl RenderWithContext<Geometry2DOutput> for BezierPolygon {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}

impl BuiltinWorkbenchDefinition for BezierPolygon {
    fn id() -> &'static str {
        "BezierPolygon"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(
                BezierPolygon {
                    path: args.get("path"),
                },
            )))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(path: String)].into_iter().collect()
    }
}
