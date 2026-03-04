// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in 2D geometries.

mod circle;
mod involute_gear;
mod line;
mod ngon;
mod rect;
mod rounded_rect;
mod sector;
mod sinusoidal_gear;
mod text;
mod thread_profile;

use microcad_lang::builtin::*;

/// Module for built-in 2D geometries.
pub fn geo2d() -> Symbol {
    crate::ModuleBuilder::new("geo2d")
        .builtin::<circle::Circle>()
        .builtin::<involute_gear::InvoluteGearProfile>()
        .builtin::<line::Line>()
        .builtin::<ngon::Ngon>()
        .builtin::<sector::Sector>()
        .builtin::<rect::Rect>()
        .builtin::<rounded_rect::RoundedRect>()
        .builtin::<sinusoidal_gear::SinusoidalGearProfile>()
        .builtin::<text::Text>()
        .builtin::<thread_profile::ThreadProfile>()
        .build()
}
