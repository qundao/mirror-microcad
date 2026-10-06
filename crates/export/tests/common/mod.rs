// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Common exporter test functions

use microcad_builtin::{BuiltinEvalContext, BuiltinPrimitiveCall, mu};
use microcad_core::Rect;
use microcad_lang_types::{Model, Value, arguments};

pub fn circle(r: f64) -> Model {
    let mut ctx = BuiltinEvalContext::default();
    mu::geo2d::Circle::call(arguments!(radius = Value::mm(r)), &mut ctx).expect("No error")
}

pub fn rect(rect: Rect) -> Model {
    let mut ctx = BuiltinEvalContext::default();
    let (width, height) = (rect.width(), rect.height());
    let (x, y) = rect.min().x_y();
    mu::geo2d::Rect::call(
        arguments!(
            x = Value::mm(x),
            y = Value::mm(y),
            width = Value::mm(width),
            height = Value::mm(height)
        ),
        &mut ctx,
    )
    .expect("No error")
}

/// Resolve target directory dynamically (works in workspaces & single crates)
pub fn target_dir() -> std::path::PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"))
}
