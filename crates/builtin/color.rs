// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in color module `__builtin::color`.

use microcad_core::{Color, Scalar};
use microcad_lang::{parameter, resolve::*, value::*};

/// Create a color from red, green and blue.
fn rgb() -> Symbol {
    Symbol::new_builtin_fn(
        "rgb",
        [
            parameter!(r: Scalar),
            parameter!(g: Scalar),
            parameter!(b: Scalar),
        ]
        .into_iter(),
        &|_params, args, _| {
            Ok(Value::Tuple(Box::new(
                Color::rgb(
                    args.get::<Scalar>("r") as f32,
                    args.get::<Scalar>("g") as f32,
                    args.get::<Scalar>("b") as f32,
                )
                .into(),
            )))
        },
        None,
    )
}

/// Create a color from red, green, blue and alpha.
fn rgba() -> Symbol {
    Symbol::new_builtin_fn(
        "rgba",
        [
            parameter!(r: Scalar),
            parameter!(g: Scalar),
            parameter!(b: Scalar),
            parameter!(a: Scalar),
        ]
        .into_iter(),
        &|_params, args, _| {
            Ok(Color::rgba(
                args.get::<Scalar>("r") as f32,
                args.get::<Scalar>("g") as f32,
                args.get::<Scalar>("b") as f32,
                args.get::<Scalar>("a") as f32,
            )
            .into())
        },
        None,
    )
}

/// Built-in color module.
pub fn color() -> Symbol {
    crate::ModuleBuilder::new("color")
        .symbol(rgb())
        .symbol(rgba())
        .pub_const("RED", Color::RED)
        .pub_const("BLUE", Color::BLUE)
        .pub_const("GREEN", Color::GREEN)
        .pub_const("YELLOW", Color::YELLOW)
        .pub_const("CYAN", Color::CYAN)
        .pub_const("MAGENTA", Color::MAGENTA)
        .pub_const("BLACK", Color::BLACK)
        .pub_const("WHITE", Color::WHITE)
        .pub_const("GRAY", Color::GRAY)
        .pub_const("ORANGE", Color::ORANGE)
        .pub_const("PURPLE", Color::PURPLE)
        .pub_const("PINK", Color::PINK)
        .pub_const("BROWN", Color::BROWN)
        .pub_const("LIME", Color::LIME)
        .pub_const("TEAL", Color::TEAL)
        .pub_const("NAVY", Color::NAVY)
        .pub_const("TRANSPARENT", Color::TRANSPARENT)
        .build()
}
