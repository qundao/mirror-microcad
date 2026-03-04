// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in color module `__builtin::color`.

use microcad_builtin_proc_macros::builtin_mod;

/// µcad built-in color module.
#[builtin_mod]
#[allow(clippy::module_inception)]
pub mod color {
    use microcad_builtin_proc_macros::builtin_fn;
    use microcad_core::{Scalar, color::Color};
    use microcad_lang::{parameter, resolve::*, value::*};

    /// Red color.
    pub const RED: Color = Color::RED;

    /// Blue color.
    pub const BLUE: Color = Color::BLUE;

    /// Green color.
    pub const GREEN: Color = Color::GREEN;

    /// Yellow color.
    pub const YELLOW: Color = Color::YELLOW;

    /// Cyan color.
    pub const CYAN: Color = Color::CYAN;

    /// Magenta color.
    pub const MAGENTA: Color = Color::MAGENTA;

    /// Black color.
    pub const BLACK: Color = Color::BLACK;

    /// White color.
    pub const WHITE: Color = Color::WHITE;

    /// Gray color.
    pub const GRAY: Color = Color::GRAY;

    /// Orange color.
    pub const ORANGE: Color = Color::ORANGE;

    /// Purple color.
    pub const PURPLE: Color = Color::PURPLE;

    /// Pink color.
    pub const PINK: Color = Color::PINK;

    /// Brown color.
    pub const BROWN: Color = Color::BROWN;

    /// Lime color.
    pub const LIME: Color = Color::LIME;

    /// Teal color.
    pub const TEAL: Color = Color::TEAL;

    /// Navy color.
    pub const NAVY: Color = Color::NAVY;

    /// Transparent (fully transparent) color.
    pub const TRANSPARENT: Color = Color::TRANSPARENT;

    /// Create a color from red, green and blue.
    #[builtin_fn(r: Scalar, g: Scalar, b: Scalar)]
    pub fn rgb() -> Symbol {
        |_params, args, _| {
            Ok(Color::rgb(
                args.get::<Scalar>("r") as f32,
                args.get::<Scalar>("g") as f32,
                args.get::<Scalar>("b") as f32,
            )
            .into())
        }
    }

    /// Create a color from red, green, blue and alpha.
    #[builtin_fn(r: Scalar, g: Scalar, b: Scalar, a: Scalar)]
    pub fn rgba() -> Symbol {
        |_params, args, _| {
            Ok(Color::rgba(
                args.get::<Scalar>("r") as f32,
                args.get::<Scalar>("g") as f32,
                args.get::<Scalar>("b") as f32,
                args.get::<Scalar>("a") as f32,
            )
            .into())
        }
    }
}
