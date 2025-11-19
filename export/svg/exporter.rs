// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use microcad_core::{Color, Scalar};
use microcad_lang::{Id, builtin::*, model::*, parameter, render::RenderError, value::*};

/// SVG Exporter.
pub struct SvgExporter;

/// A theme for SVG export.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Background color of the drawing canvas.
    pub background: Color,
    /// Color used for grid lines.
    pub grid: Color,
    /// Color used for selected entities.
    pub selection: Color,
    /// Color used for highlighting hovered entities.
    pub highlight: Color,
    /// Default color for entities.
    pub entity: Color,
    /// Default color for entity outlines.
    pub outline: Color,
    /// Color used for active construction lines.
    pub active: Color,
    /// Color used for inactive construction lines.
    pub inactive: Color,
    /// Color for dimensions and annotations.
    pub measure: Color,
    /// Color for snapping indicators.
    pub snap_indicator: Color,
    /// Color for guidelines (e.g. inference lines).
    pub guide: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::rgb(1.0, 1.0, 1.0),
            grid: Color::rgb(0.85, 0.85, 0.85),
            selection: Color::rgb(0.0, 0.4, 0.8),
            highlight: Color::rgb(1.0, 0.6, 0.0),
            entity: Color::rgba(0.7, 0.7, 0.7, 0.7),
            outline: Color::rgb(0.1, 0.1, 0.1),
            active: Color::rgb(0.2, 0.2, 0.2),
            inactive: Color::rgb(0.8, 0.8, 0.8),
            measure: Color::rgb(0.0, 0.8, 0.8),
            snap_indicator: Color::rgb(0.0, 0.8, 0.8),
            guide: Color::rgb(0.6, 0.6, 0.6),
        }
    }
}

/// Settings for this exporter.
pub struct SvgExporterSettings {
    /// Relative padding (e.g. 0.05 = 5% = padding on each side).
    padding_factor: Scalar,
}

impl Default for SvgExporterSettings {
    fn default() -> Self {
        Self {
            padding_factor: 0.05, // 5% padding on each side.
        }
    }
}

impl SvgExporter {
    /// Generate SVG style string from theme.
    pub fn theme_to_svg_style(theme: &Theme) -> String {
        fn fill_stroke_style(
            class_name: &str,
            fill_color: Color,
            stroke_color: Color,
            stroke_width: Scalar,
        ) -> String {
            format!(
                r#" 
        .{class_name} {{
            fill: {fill_color};
            stroke: {stroke_color};
            stroke-width: {stroke_width};
        }}
        "#,
                fill_color = fill_color.to_svg_color(),
                stroke_color = stroke_color.to_svg_color()
            )
        }

        fn fill_style(class_name: &str, fill: Color) -> String {
            format!(
                r#" 
        .{class_name}-fill {{
            fill: {fill};
            stroke: none;
        }}
        "#,
                fill = fill.to_svg_color()
            )
        }

        fn stroke_style(class_name: &str, stroke: Color, stroke_width: Scalar) -> String {
            format!(
                r#" 
        .{class_name}-stroke {{
            fill: none;
            stroke: {stroke};
            stroke-width: {stroke_width};
        }}
        "#,
                stroke = stroke.to_svg_color()
            )
        }

        let mut style = [
            ("background", theme.background, None),
            ("grid", theme.grid, Some(0.2)),
            ("measure", theme.measure, Some(0.2)),
            ("highlight", theme.highlight, Some(0.2)),
        ]
        .into_iter()
        .fold(String::new(), |mut style, item| {
            if let Some(stroke) = item.2 {
                style += &fill_stroke_style(item.0, item.1, item.1, stroke);
                style += &stroke_style(item.0, item.1, stroke)
            }
            style += &fill_style(item.0, item.1);
            style
        });

        style += &fill_stroke_style("entity", theme.entity, theme.outline, 0.4);

        style += r#"
            .active { fill-opacity: 1.0; stroke-opacity: 1.0; }
            .inactive { fill-opacity: 0.3; stroke-opacity: 0.3; }
        "#;

        style
    }
}

impl Exporter for SvgExporter {
    fn model_parameters(&self) -> microcad_lang::value::ParameterValueList {
        [
            parameter!(style: String = String::new()),
            parameter!(fill: String = String::new()),
        ]
        .into_iter()
        .collect()
    }

    fn export(&self, model: &Model, filename: &std::path::Path) -> Result<Value, ExportError> {
        use crate::svg::*;
        use microcad_core::CalcBounds2D;
        let settings = SvgExporterSettings::default();
        let bounds = model.calc_bounds_2d();

        if bounds.is_valid() {
            let content_rect = bounds
                .enlarge(2.0 * settings.padding_factor)
                .rect()
                .expect("Rect");
            log::debug!("Exporting into SVG file {filename:?}");
            let f = std::fs::File::create(filename)?;
            let mut writer = SvgWriter::new_canvas(
                Box::new(std::io::BufWriter::new(f)),
                model.get_size(),
                content_rect,
                None,
            )?;
            writer.style(&SvgExporter::theme_to_svg_style(&Theme::default()))?;

            model.write_svg(&mut writer, &SvgTagAttributes::default())?;
            Ok(Value::None)
        } else {
            Err(ExportError::RenderError(RenderError::NothingToRender))
        }
    }

    fn output_type(&self) -> OutputType {
        OutputType::Geometry2D
    }
}

impl FileIoInterface for SvgExporter {
    fn id(&self) -> Id {
        Id::new("svg")
    }
}
