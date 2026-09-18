// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for rendering model trees.

use microcad_builtin::{__mu, BuiltinEvalContext, BuiltinPrimitiveCall};
use microcad_lang_types::{
    ModelTree, Value, arguments,
    model::{
        self,
        Element::{self, BuiltinWorkpiece},
    },
};
use microcad_render::{Render, RenderContext};

/// Circle(42mm);
#[test]
fn render_circle() {
    let mut ctx = BuiltinEvalContext::default();
    let circle =
        microcad_builtin::mu::geo2d::Circle::call(arguments!(radius = Value::mm(42.0)), &mut ctx)
            .expect("No error");

    let model_tree = ModelTree::new(circle);

    let mut ctx = RenderContext::new(&model_tree);
    let geometry = model_tree.render(&mut ctx).expect("No render errors");

    insta::assert_snapshot!("render_circle", geometry);
}
