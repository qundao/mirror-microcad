// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in color module `__builtin::color`.

use microcad_builtin_proc_macros::builtin_mod;
use microcad_core::Color;

#[builtin_mod]
pub mod color {
    use microcad_core::Scalar;
    use microcad_core::color::Color;
    use microcad_lang::{parameter, resolve::*, value::*};

    pub const RED: Color = Color::RED;
    pub const BLUE: Color = Color::BLUE;
    pub const GREEN: Color = Color::GREEN;
    pub const YELLOW: Color = Color::YELLOW;
    pub const CYAN: Color = Color::CYAN;
    pub const MAGENTA: Color = Color::MAGENTA;
    pub const BLACK: Color = Color::BLACK;
    pub const WHITE: Color = Color::WHITE;
    pub const GRAY: Color = Color::GRAY;
    pub const ORANGE: Color = Color::ORANGE;
    pub const PURPLE: Color = Color::PURPLE;
    pub const PINK: Color = Color::PINK;
    pub const BROWN: Color = Color::BROWN;
    pub const LIME: Color = Color::LIME;
    pub const TEAL: Color = Color::TEAL;
    pub const NAVY: Color = Color::NAVY;
    pub const TRANSPARENT: Color = Color::TRANSPARENT;

    /// Create a color from red, green and blue.
    pub fn rgb() -> Symbol {
        Symbol::new_builtin_fn(
            "rgb",
            [
                parameter!(r: Scalar),
                parameter!(g: Scalar),
                parameter!(b: Scalar),
            ]
            .into_iter(),
            &|_params, args, _| {
                Ok(Color::rgb(
                    args.get::<Scalar>("r") as f32,
                    args.get::<Scalar>("g") as f32,
                    args.get::<Scalar>("b") as f32,
                )
                .into())
            },
            None,
        )
    }

    /// Create a color from red, green, blue and alpha.
    pub fn rgba() -> Symbol {
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
}
