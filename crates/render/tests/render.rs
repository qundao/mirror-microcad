// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for rendering model trees.

use microcad_builtin::{BuiltinEvalContext, BuiltinPrimitiveCall, mu};
use microcad_lang_types::{Model, ModelTree, Value, arguments, model::Element};
use microcad_render::{Render, RenderContext};

fn circle(r: f64) -> Model {
    let mut ctx = BuiltinEvalContext::default();
    mu::geo2d::Circle::call(arguments!(radius = Value::mm(r)), &mut ctx).expect("No error")
}

/// Circle(42mm);
#[test]
fn render_circle() {
    let circle = circle(42.0);
    let model_tree = ModelTree::new(circle);

    let mut ctx = RenderContext::new(&model_tree);
    let geometry = model_tree.render(&mut ctx).expect("No render errors");

    insta::assert_snapshot!("render_circle", geometry);
}

/// Circle(42mm) - Circle(23mm);
#[test]
fn render_difference() {
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
    let geometry = diff.render(&mut ctx).expect("No render errors");

    insta::assert_snapshot!("render_difference", geometry);
}
