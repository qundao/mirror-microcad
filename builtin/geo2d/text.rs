// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use geo::Centroid;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Text geometry with a height.
#[derive(Debug, Clone)]
pub struct Text {
    /// Text height.
    pub height: Scalar,

    /// Text.
    pub text: String,

    /// Font file (*.ttf or *.otf).
    pub font_file: String,
}

impl Render<Geometry2D> for Text {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        let font_data = if self.font_file.is_empty() {
            Vec::from(include_bytes!("../assets/fira-code-mono.ttf"))
        } else {
            std::fs::read(&self.font_file).expect("Failed to read font file")
        };

        // Load the font into rusttype
        let font = rusttype::Font::try_from_bytes(font_data.as_slice())
            .expect("Failed to load font into rusttype");

        let options = geo_rusttype::TextOptions::new(self.height as f32, font, None, None);

        use geo::Simplify;
        let polygons = geo_rusttype::text_to_multi_polygon(&self.text, options)
            .simplify(resolution.linear * 0.5);

        if let Some(center) = polygons.centroid() {
            use microcad_core::traits::Align;
            Geometry2D::MultiPolygon(
                polygons.reflect_2d(&Line(center, center + Point::new(1.0, 0.0))),
            )
            .align()
        } else {
            Geometries2D::default().into()
        }
    }
}

impl RenderWithContext<Geometry2DOutput> for Text {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}

impl BuiltinWorkbenchDefinition for Text {
    fn id() -> &'static str {
        "Text"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Text {
                height: args.get("height"),
                text: args.get("text"),
                font_file: args.get("font_file"),
            })))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(height: Scalar),
            parameter!(text: String),
            parameter!(font_file: String = String::new()),
        ]
        .into_iter()
        .collect()
    }
}
