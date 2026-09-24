// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export tests

use std::str::FromStr as _;

use geo::{Translate, coord};
use microcad_core::*;
use microcad_export::svg::{
    CenteredText, MapToCanvas, SvgTagAttribute, SvgTagAttributes, SvgWriter, WriteSvg,
    WriteSvgMapped,
};

use crate::common::assert_svg_snapshot;

mod common;

#[test]
fn svg_writer() -> std::io::Result<()> {
    assert_svg_snapshot(
        "svg_writer",
        SvgWriter::new(
            vec![],
            None,
            Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: 100.0, y: 100.0}),
            None,
        ),
        |writer| {
            writer.with_attr([("style", "fill:blue")], |writer| {
                geo::Rect::new(geo::Point::new(10.0, 10.0), geo::Point::new(20.0, 20.0))
                    .write_svg(writer)
            })?;

            writer.with_attr([("style", "fill:red")], |writer| {
                geo2d::Circle {
                    radius: 10.0,
                    offset: Vec2::new(50.0, 50.0),
                }
                .write_svg(writer)
            })?;

            writer.with_attr([("style", "stroke:black;")], |writer| {
                Line(geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0)).write_svg(writer)
            })?;

            writer.with_attr([("style", "stroke:black;")], |writer| {
                Line(geo::Point::new(100.0, 0.0), geo::Point::new(0.0, 100.0))
                    .shorter(6.0)
                    .write_svg(writer)
            })
        },
    )
}

#[test]
fn svg_canvas() -> std::io::Result<()> {
    let mut content_rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: 100.0, y: 100.0});

    assert_svg_snapshot(
        "svg_canvas",
        SvgWriter::new(
            vec![],
            Size2::A4.transposed().into(),
            content_rect.clone(),
            Some(2.0),
        ),
        |writer| {
            writer.with_attr(
                SvgTagAttribute::style(
                    Some(Color::from_str("none").expect("transparent")),
                    Some(Color::from_str("black").expect("Black color")),
                    Some(1.0),
                ),
                |writer| content_rect.write_svg_mapped(writer),
            )?;

            [
                (0.0, 0.0),
                (0.0, 100.0),
                (100.0, 0.0),
                (100.0, 100.0),
                (50.0, 50.0),
            ]
            .iter()
            .map(|p| Circle {
                radius: 2.0,
                offset: Vec2::new(p.0, p.1),
            })
            .try_for_each(|c| {
                let mut orig_c = c.clone();
                writer.with_attr(
                    SvgTagAttribute::style(
                        Some(Color::from_str("blue").expect("Color")),
                        None,
                        None,
                    ),
                    |writer| orig_c.write_svg_mapped(writer),
                )?;

                let p = Point::new(c.offset.x, c.offset.y);
                writer.with_attr(
                    SvgTagAttribute::style(
                        Some(Color::from_str("gray").expect("Color")),
                        Some(Color::from_str("none").expect("Color")),
                        None,
                    ),
                    |writer| {
                        CenteredText {
                            text: format!("({}mm,{}mm)", p.x(), p.y()),
                            rect: Rect::new(p, p),
                            font_size: 1.0,
                        }
                        .write_svg_mapped(writer)
                    },
                )
            })
        },
    )
}

/*
#[test]
fn svg_sample_sketch() -> std::io::Result<()> {

    assert_svg_snapshot(
        "svg_canvas",
        SvgWriter::new(
            vec![],
            Size2::A4.transposed().into(),
            Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: 50.0, y: 50.0}),
            Some(2.0),
        ),
        |writer| {#
}


    let content_rect = ;
    let mut svg = SvgWriter::new(
        Box::new(file),
        Size2::A4.transposed().into(),
        content_rect,
        Some(3.0),
    )
    .expect("test error");

    svg.style(&SvgExporter::theme_to_svg_style(&Theme::default()))?;

    let radius = 10.0;
    let width = 30.0;
    let height = 20.0;

    let rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: width, y: height});
    let circle = Circle {
        radius,
        offset: Vec2::new(width, height),
    };

    Background.write_svg(&mut svg, &Default::default())?;
    Grid::default().write_svg(&mut svg, &Default::default())?;

    rect.write_svg_mapped(
        &mut svg,
        &SvgTagAttribute::class("entity-stroke inactive").into(),
    )?;

    CenteredText {
        text: "r".into(),
        rect,
        font_size: 4.0,
    }
    .write_svg_mapped(
        &mut svg,
        &SvgTagAttribute::class("entity-fill inactive").into(),
    )?;

    // Draw rectangle measures

    // Height measure for rect.
    EdgeLengthMeasure::height(&rect, 10.0, Some("height")).write_svg_mapped(
        &mut svg,
        &[
            SvgTagAttribute::class("measure"),
            SvgTagAttribute::class("inactive"),
        ]
        .into_iter()
        .collect(),
    )?;
    // Width measure for rect.
    EdgeLengthMeasure::width(&rect, 10.0, Some("width")).write_svg_mapped(
        &mut svg,
        &[
            SvgTagAttribute::class("measure"),
            SvgTagAttribute::class("inactive"),
        ]
        .into_iter()
        .collect(),
    )?;

    // Draw circle `c`.
    circle.write_svg_mapped(
        &mut svg,
        &[SvgTagAttribute::class("entity-stroke inactive")]
            .into_iter()
            .collect(),
    )?;
    CenteredText {
        text: "c".into(),
        rect: circle.calc_bounds_2d().rect().expect("Rect"),
        font_size: 4.0,
    }
    .write_svg_mapped(
        &mut svg,
        &[
            SvgTagAttribute::class("entity-fill"),
            SvgTagAttribute::class("inactive"),
        ]
        .into_iter()
        .collect(),
    )?;

    RadiusMeasure::new(circle.clone(), Some("radius".into()), None).write_svg_mapped(
        &mut svg,
        &[
            SvgTagAttribute::class("measure"),
            SvgTagAttribute::class("inactive"),
        ]
        .into_iter()
        .collect(),
    )?;

    // Draw intersection.
    let intersection = Geometry2D::Rect(rect).boolean_op(
        Geometry2D::Polygon(
            Circle::circle_polygon(circle.radius, 32).translate(circle.offset.x, circle.offset.y),
        ),
        BooleanOp::Intersect,
    );

    intersection.write_svg_mapped(
        &mut svg,
        &[SvgTagAttribute::class("entity-stroke active")]
            .into_iter()
            .collect(),
    )?;

    SizeMeasure::bounds(&intersection).write_svg_mapped(
        &mut svg,
        &[
            SvgTagAttribute::class("measure"),
            SvgTagAttribute::class("active"),
        ]
        .into_iter()
        .collect(),
    )?;

    Ok(())
}
*/
