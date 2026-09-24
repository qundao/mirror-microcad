// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! SVG tests
use cgmath::Vector2;
use geo::coord;
use microcad_builtin::{BuiltinEvalContext, BuiltinPrimitiveCall, mu};
use microcad_core::{Circle, Rect};
use microcad_export::svg::{SvgWriter, WriteSvg};
use microcad_lang_types::{Model, ModelTree, Value, arguments, model::Element};
use microcad_render::RenderContext;

use crate::common::assert_svg_snapshot;

mod common;

/// Render a polygon with holes
#[test]
fn svg_polygon() -> std::io::Result<()> {
    let bounds = Rect::new(coord! { x: 0., y: 0. }, coord! { x: 100., y: 100. });

    assert_svg_snapshot(
        "svg_polygon",
        SvgWriter::new(vec![], None, bounds, None),
        |writer| {
            let circle = Circle {
                radius: 10.0,
                offset: Vector2::new(25.0, 25.0),
            }
            .to_polygon(32);

            circle.write_svg(writer)
        },
    )
}

fn circle(r: f64) -> Model {
    let mut ctx = BuiltinEvalContext::default();
    mu::geo2d::Circle::call(arguments!(radius = Value::mm(r)), &mut ctx).expect("No error")
}

// Circle(42mm) - Circle(23mm);
#[test]
fn difference() -> std::io::Result<()> {
    let bounds = Rect::new(coord! { x: 0., y: 0. }, coord! { x: 100., y: 100. });

    assert_svg_snapshot(
        "svg_difference",
        SvgWriter::new(vec![], None, bounds, None),
        |writer| {
            let mut ctx = BuiltinEvalContext::default();

            // {
            //     Circle(42mm);
            //     Circle(23mm);
            // }
            let mut group = ModelTree::new(Element::Group);
            group.append(circle(42.0));
            group.append(circle(23.0));
            let diff = mu::ops::difference(arguments!(self = group), &mut ctx).expect("No error");

            let mut ctx = RenderContext::new(&diff);
            let geometry = microcad_render::render(&diff, &mut ctx).expect("No render errors");

            geometry.write_svg(writer)
        },
    )
}
