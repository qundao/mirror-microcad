// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod circle;
mod involute_gear;
mod line;
mod ngon;
mod pie;
mod rect;
mod rounded_rect;
mod text;
mod thread_profile;

pub use circle::*;
pub use involute_gear::*;
pub use line::*;
pub use ngon::*;
pub use pie::*;
pub use rect::*;
pub use rounded_rect::*;
pub use text::*;
pub use thread_profile::*;

use microcad_lang::builtin::*;

/// Module for built-in 2D geometries.
pub fn geo2d() -> Symbol {
    crate::ModuleBuilder::new("geo2d")
        .builtin::<Circle>()
        .builtin::<InvoluteGearProfile>()
        .builtin::<Line>()
        .builtin::<Ngon>()
        .builtin::<Pie>()
        .builtin::<Rect>()
        .builtin::<RoundedRect>()
        .builtin::<Text>()
        .builtin::<ThreadProfile>()
        .build()
}
