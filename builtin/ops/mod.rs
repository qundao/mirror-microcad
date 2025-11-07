// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::builtin::*;

mod align;
mod buffer;
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
        .builtin::<align::Align>()
        .builtin::<buffer::Buffer>()
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
