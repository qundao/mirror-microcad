// Copyright © 2024-2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in operations.

use microcad_lang::builtin::*;

mod align;
mod buffer;
mod center;
mod distribute_grid;
mod extrude;
mod hull;
mod mirror;
mod orient;
mod reflect;
mod revolve;
mod rotate;
mod scale;
mod spiralize;
mod translate;

/// Creates the builtin `operation` module
pub fn ops() -> Symbol {
    crate::ModuleBuilder::new("ops")
        .builtin::<operation::Union>()
        .builtin::<operation::Subtract>()
        .builtin::<operation::Intersect>()
        .builtin::<operation::Multiply>()
        .builtin::<align::Align>()
        .builtin::<buffer::Buffer>()
        .builtin::<center::Center>()
        .builtin::<distribute_grid::DistributeGrid>()
        .builtin::<hull::Hull>()
        .builtin::<extrude::Extrude>()
        .builtin::<mirror::Mirror>()
        .builtin::<orient::Orient>()
        .builtin::<reflect::Reflect>()
        .builtin::<revolve::Revolve>()
        .builtin::<rotate::Rotate>()
        .builtin::<scale::Scale>()
        .builtin::<spiralize::Spiralize>()
        .builtin::<translate::Translate>()
        .build()
}
