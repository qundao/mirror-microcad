// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Experimental built-in features

mod bezier;

use microcad_lang::builtin::*;

/// Module for built-in 2D geometries.
pub fn lab() -> Symbol {
    crate::ModuleBuilder::new("lab")
        .builtin::<bezier::BezierPolygon>()
        .build()
}
